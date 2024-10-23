use std::io::Read;
use std::io::Seek;
use std::ops::Deref;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use vfs::error::VfsErrorKind;
use vfs::FileSystem;

use crate::consts::BLOCK_SIZE;
use crate::StfsEntry;
use crate::StfsEntryRef;
use crate::StfsPackage;

pub struct StfsFileReader<T> {
	pub block_ranges: Vec<Range<u64>>,
	pub block_position: u64,
	pub block_idx: usize,
	pub position: u64,
	pub data: Arc<StfsDataWrapper<T>>,
	pub len: u64,
}

impl<T> StfsFileReader<T> {
	fn recalculate_block(&mut self) {
		if self.position >= self.len {
			// do nothing, reads will return EOF
			return;
		}

		self.block_idx = usize::try_from(self.position).unwrap() / BLOCK_SIZE;
		self.block_position = self.position % u64::try_from(BLOCK_SIZE).unwrap();
	}
}

impl<T> Seek for StfsFileReader<T>
where
	T: AsRef<[u8]>,
{
	fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
		let (base, delta) = match pos {
			std::io::SeekFrom::Start(pos) => {
				self.position = pos;
				self.recalculate_block();
				return Ok(self.position);
			}
			std::io::SeekFrom::End(pos) => (self.data.as_slice().len() as u64, pos),
			std::io::SeekFrom::Current(delta) => (self.position, delta),
		};

		match base.checked_add_signed(delta) {
			Some(n) => {
				self.position = std::cmp::min(self.len, n);
				self.recalculate_block();
				Ok(self.position)
			}
			None => Err(std::io::Error::new(
				std::io::ErrorKind::InvalidInput,
				"invalid seek to a negative or overflowing position",
			)),
		}
	}
}

impl<T> Read for StfsFileReader<T>
where
	T: AsRef<[u8]>,
{
	fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
		let mut output_bytes_remaining = buf.len();
		let mut bytes_read = 0;

		if self.block_idx >= self.block_ranges.len() || self.position >= self.len {
			return Ok(0);
		}

		for (idx, mapping) in self.block_ranges.iter().enumerate().skip(self.block_idx) {
			let block_len = mapping.end - mapping.start;
			let (mapping_start, block_remaining_len) = if idx == self.block_idx {
				(usize::try_from(mapping.start + self.block_position).unwrap(), block_len - self.block_position)
			} else {
				(0, block_len)
			};

			let bytes_to_copy = std::cmp::min(self.len - self.position, block_remaining_len);
			let bytes_to_copy = std::cmp::min(output_bytes_remaining, usize::try_from(bytes_to_copy).unwrap());

			buf[..bytes_to_copy].copy_from_slice(&self.data.as_slice()[mapping_start..(mapping_start + bytes_to_copy)]);
			buf = &mut buf[bytes_to_copy..];
			bytes_read += bytes_to_copy;
			println!("{} {}", bytes_to_copy, output_bytes_remaining);
			output_bytes_remaining -= bytes_to_copy;

			// Quit reading if we've read all data requested or have reached EOF
			if output_bytes_remaining == 0
				|| (idx == self.block_ranges.len() - 1
					&& mapping_start + bytes_to_copy == mapping_start + usize::try_from(block_len).unwrap())
			{
				self.position += u64::try_from(bytes_to_copy).unwrap();
				self.recalculate_block();

				break;
			}
		}

		Ok(bytes_read)
	}
}

#[derive(Clone)]
pub struct StfsDataWrapper<T> {
	/// Do not use directly. STFS filesystems may have a different start offset, and the caller may be handing us the
	/// original file for a variety of reasons.
	data: Arc<T>,
	filesystem_start: usize,
}

impl<T: AsRef<[u8]>> StfsDataWrapper<T> {
	fn as_slice(&self) -> &[u8] {
		&self.data.deref().as_ref()[self.filesystem_start..]
	}
}

impl<T> std::fmt::Debug for StfsDataWrapper<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("StfsDataWrapper").field("data", &"...").field("data_start", &self.filesystem_start).finish()
	}
}

pub struct StFS<T> {
	files: StfsEntryRef,
	data: Arc<StfsDataWrapper<T>>,
}

impl<T> std::fmt::Debug for StFS<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("StFS").field("files", &self.files).field("data", &self.data).finish()
	}
}

impl<T: Clone> Clone for StFS<T> {
	fn clone(&self) -> Self {
		Self { files: self.files.clone(), data: self.data.clone() }
	}
}

impl<T> StFS<T> {
	pub fn from_raw_parts(package: &StfsPackage, data: Arc<T>, filesystem_start: usize) -> StFS<T> {
		Self { files: Arc::clone(&package.files), data: Arc::new(StfsDataWrapper { data, filesystem_start }) }
	}
}

impl<T> StFS<T> {
	fn find_file(&self, path: &str) -> vfs::VfsResult<StfsEntryRef> {
		let path = PathBuf::from(path);
		let mut current = Arc::clone(&self.files);

		for part in path.iter() {
			if part == "/" {
				continue;
			}
			// Look up this part of the path in our dir
			let current_copy = Arc::clone(&current);
			let node = current_copy.lock();

			match &*node {
				crate::StfsEntry::File(_, _) => return Err(VfsErrorKind::FileNotFound.into()),
				crate::StfsEntry::Folder { entry, files } => {
					if let Some(node) = files.iter().find(|file| file.lock().name() == part.to_string_lossy()) {
						current = Arc::clone(node)
					} else {
						return Err(VfsErrorKind::FileNotFound.into());
					}
				}
			}
		}

		Ok(current)
	}
}

impl<T: AsRef<[u8]> + Send + Sync + 'static> FileSystem for StFS<T> {
	fn read_dir(&self, path: &str) -> vfs::VfsResult<Box<dyn Iterator<Item = String> + Send>> {
		let dir = self.find_file(path)?;

		let dir = dir.lock();

		if let StfsEntry::Folder { entry: _, files } = &*dir {
			Ok(Box::new(files.iter().map(|file| file.lock().name()).collect::<Vec<_>>().into_iter()))
		} else {
			unreachable!("we should never have a file here")
		}
	}

	fn create_dir(&self, path: &str) -> vfs::VfsResult<()> {
		todo!()
	}

	fn open_file(&self, path: &str) -> vfs::VfsResult<Box<dyn vfs::SeekAndRead + Send>> {
		let file = self.find_file(path)?;
		let file = file.lock();
		let file_info = file.file_ref().unwrap();
		println!("opening file {}", path);
		Ok(Box::new(StfsFileReader {
			block_ranges: file_info.1.clone(),
			block_position: 0,
			block_idx: 0,
			position: 0,
			data: Arc::clone(&self.data),
			len: u64::from(file_info.0.file_attributes.as_ref().unwrap().file_size),
		}))
	}

	fn create_file(&self, path: &str) -> vfs::VfsResult<Box<dyn vfs::SeekAndWrite + Send>> {
		todo!()
	}

	fn append_file(&self, path: &str) -> vfs::VfsResult<Box<dyn vfs::SeekAndWrite + Send>> {
		todo!()
	}

	fn metadata(&self, path: &str) -> vfs::VfsResult<vfs::VfsMetadata> {
		let file = self.find_file(path)?;
		let file = &*file.lock();

		let metadata = match file {
			StfsEntry::File(entry, _) => {
				let attr = entry.file_attributes.as_ref().unwrap();
				vfs::VfsMetadata {
					file_type: vfs::VfsFileType::File,
					len: attr.file_size as u64,
					created: Some(crate::util::stf_timestamp_to_chrono(attr.created_time_stamp).into()),
					modified: None,
					accessed: Some(crate::util::stf_timestamp_to_chrono(attr.access_time_stamp).into()),
				}
			}
			StfsEntry::Folder { entry, files } => {
				let attr = entry.file_attributes.as_ref().unwrap();
				vfs::VfsMetadata {
					file_type: vfs::VfsFileType::Directory,
					len: 0,
					created: Some(crate::util::stf_timestamp_to_chrono(attr.created_time_stamp).into()),
					modified: None,
					accessed: Some(crate::util::stf_timestamp_to_chrono(attr.access_time_stamp).into()),
				}
			}
		};

		Ok(metadata)
	}

	fn exists(&self, path: &str) -> vfs::VfsResult<bool> {
		Ok(self.find_file(path).is_ok())
	}

	fn remove_file(&self, path: &str) -> vfs::VfsResult<()> {
		todo!()
	}

	fn remove_dir(&self, path: &str) -> vfs::VfsResult<()> {
		todo!()
	}
}
