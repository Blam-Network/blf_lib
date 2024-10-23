use binrw::binrw;
use binrw::BinReaderExt;
use binrw::NullWideString;
use binrw::PosValue;
use sha1::Digest;
use sha1::Sha1;
use std::sync::Arc;
use stfs::StfsPackage;
use stfs::StfsVolumeDescriptor;
use vfs::VfsPath;

use serde::Serialize;
use std::io::Cursor;
use variantly::Variantly;

use xecrypt::XContentKeyMaterial;
use xecrypt::XContentSignatureType;

use crate::error::XContentError;
use crate::util::*;

const MAX_IMAGE_SIZE: usize = 0x4000;

#[derive(Debug, Serialize)]
#[binrw]
#[br(import(is_profile_embedded_content: bool))]
pub enum XContentHeaderMetadata {
	#[br(pre_assert(!is_profile_embedded_content))]
	XContentPackage(XContentHeader),
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct FixedLengthNullWideString(
	#[brw(pad_size_to = 128)]
	#[serde(serialize_with = "serialize_null_wide_string")]
	NullWideString,
);

impl std::ops::Deref for FixedLengthNullWideString {
	type Target = NullWideString;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Debug, Serialize)]
#[binrw]
#[brw(big)]
pub struct XContentHeader {
	pub signature_type: XContentSignatureType,

	#[br(args(signature_type), pad_size_to = 0x228)]
	pub key_material: XContentKeyMaterial,

	#[bw(ignore)]
	#[serde(skip)]
	pub license_data_pos: PosValue<()>,
	pub license_data: [LicenseEntry; 0x10],
	/// Content ID is the hash of the metadata and all headers below it.
	pub content_id: [u8; 0x14],
	pub header_size: u32,

	#[serde(skip)]
	#[bw(ignore)]
	pub end_of_header_pos: PosValue<()>,

	#[br(args(header_size))]
	pub metadata: XContentMetadata,
}

impl XContentHeader {
	/// Returns the offset for data start in the XContent package. i.e. the first
	/// writable offset after all of the XContent headers.
	///
	/// For an STFS package, this will be the start of the first hash table.
	pub fn data_start_offset(&self) -> usize {
		((self.header_size as usize) + 0x0FFF) & 0xFFFF_F000
	}

	/// Returns the hash of the data covered by the RSA signature. This covers the
	pub fn header_hash(&self, data: &[u8]) -> [u8; 20] {
		let signature_start_pos = self.license_data_pos.pos as usize;
		let signature_end_pos = self.end_of_header_pos.pos as usize;

		let mut hasher = Sha1::new();
		hasher.update(&data[signature_start_pos..signature_end_pos]);

		hasher.finalize().into()
	}
}

#[derive(Debug, Serialize)]
#[binrw]
#[br(import(header_size: u32))]
pub struct XContentMetadata {
	pub content_type: ContentType,
	pub metadata_version: u32,
	pub content_size: u64,
	pub media_id: u32,
	pub version: u32,
	pub base_version: u32,
	pub title_id: u32,
	pub platform: u8,
	pub executable_type: u8,
	pub disc_number: u8,
	pub disc_in_set: u8,
	pub savegame_id: u32,
	pub console_id: [u8; 5],
	pub creator_xuid: u64,

	#[brw(seek_before = std::io::SeekFrom::Start(0x3a9))]
	pub volume_kind: FileSystemKind,

	#[brw(seek_before = std::io::SeekFrom::Start(0x379))]
	#[br(args(volume_kind))]
	pub volume_descriptor: FileSystem,

	// Start metadata v1
	pub data_file_count: u32,
	pub data_file_combined_size: u64,

	// TODO: parse the inbetween data
	#[brw(seek_before = std::io::SeekFrom::Start(0x3fd))]
	pub device_id: [u8; 0x14],

	// TODO: support localized names
	pub display_name: [FixedLengthNullWideString; 12],

	#[brw(seek_before = std::io::SeekFrom::Start(0xd11))]
	pub display_description: [FixedLengthNullWideString; 12],

	#[serde(serialize_with = "serialize_null_wide_string")]
	#[brw(seek_before = std::io::SeekFrom::Start(0x1611))]
	pub publisher_name: NullWideString,

	#[serde(serialize_with = "serialize_null_wide_string")]
	#[brw(seek_before = std::io::SeekFrom::Start(0x1691))]
	pub title_name: NullWideString,

	#[brw(seek_before = std::io::SeekFrom::Start(0x1711))]
	pub transfer_flags: u8,
	pub thumbnail_image_size: u32,
	pub title_thumbnail_image_size: u32,

	#[br(count = thumbnail_image_size)]
	#[brw(pad_size_to(MAX_IMAGE_SIZE))]
	pub thumbnail_image: Vec<u8>,

	#[br(count = title_thumbnail_image_size)]
	#[brw(pad_size_to(MAX_IMAGE_SIZE))]
	pub title_image: Vec<u8>,

	#[br(if(((header_size + 0xFFF) & 0xFFFFF000) - 0x971A > 0x15F4))]
	pub installer_type: Option<InstallerType>,
	// #[br(if(installer_type.is_some()), args(installer_type.unwrap()))]
	// pub installer_meta: Option<InstallerMeta>,
	// #[br(if(content_type.has_content_metadata()), args(content_type))]
	// pub content_metadata: Option<ContentMetadata>,
}

#[derive(Debug, Serialize, Variantly)]
pub enum XboxFilesystem {
	Stfs(StfsPackage),
}

#[derive(Debug, Serialize)]
pub struct XContentPackage {
	pub header: XContentHeader,
	pub inner_file_system: XboxFilesystem,
}

impl XContentPackage {
	pub fn to_vfs_path<T>(&self, data: Arc<T>) -> VfsPath
	where
		T: AsRef<[u8]> + Send + Sync + 'static,
	{
		match &self.inner_file_system {
			XboxFilesystem::Stfs(package) => {
				stfs::fs::StFS::from_raw_parts(package, data, self.header.data_start_offset()).into()
			}
		}
	}

	pub fn to_vfs<T>(&self, data: Arc<T>) -> Box<dyn vfs::FileSystem>
	where
		T: AsRef<[u8]> + Send + Sync + 'static,
	{
		match &self.inner_file_system {
			XboxFilesystem::Stfs(package) => {
				Box::new(stfs::fs::StFS::from_raw_parts(package, data, self.header.data_start_offset()))
			}
		}
	}

	/// Attempts to verify this package's signature.
	///
	/// Upon success this function will return the kind of console this package is signed for.
	pub fn verify_signature(&self, data: &[u8]) -> Result<xecrypt::ConsoleKind, xecrypt::Error> {
		xecrypt::verify_xcontent_signature(
			self.header.signature_type,
			&self.header.key_material,
			&self.header.header_hash(data),
		)
	}

	/// Returns the expected storage path for this file
	pub fn storage_path(&self) -> String {
		format!(
			"Content/{:016X}/{:08X}/{:08X}/{}",
			self.header.metadata.creator_xuid,
			self.header.metadata.title_id,
			self.header.metadata.content_type as u32,
			self.header.content_id.iter().map(|b| format!("{:02X}", b)).collect::<String>(),
		)
	}
}

impl TryFrom<&[u8]> for XContentPackage {
	type Error = XContentError;

	fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
		let mut cursor = Cursor::new(input);
		let header = cursor.read_be::<XContentHeader>()?;

		let inner_fs = match &header.metadata.volume_descriptor {
			FileSystem::Stfs(volume_descriptor) => {
				let start_offset = header.data_start_offset();
				let stfs_input = &input[start_offset..];
				let mut package = StfsPackage::from_volume_descriptor(volume_descriptor.clone())?;
				package.load_from_complete_file(stfs_input)?;

				XboxFilesystem::Stfs(package)
			}
			FileSystem::Svod(_) => todo!(),
			FileSystem::Fatx => todo!(),
		};

		let package = XContentPackage { header, inner_file_system: inner_fs };

		Ok(package)
	}
}

#[derive(Debug, Serialize, Copy, Clone, Eq, PartialEq)]
#[binrw]
#[brw(repr = u32)]
enum AssetSubcategory {
	CarryableCarryable = 0x44c,
	// CarryableFirst = 0x44c,
	// CarryableLast = 0x44c,
	CostumeCasualSuit = 0x68,
	CostumeCostume = 0x69,
	// CostumeFirst = 100,
	CostumeFormalSuit = 0x67,
	// CostumeLast = 0x6a,
	CostumeLongDress = 0x65,
	CostumeShortDress = 100,
	EarringsDanglers = 0x387,
	// EarringsFirst = 900,
	EarringsLargehoops = 0x38b,
	// EarringsLast = 0x38b,
	EarringsSingleDangler = 0x386,
	EarringsSingleLargeHoop = 0x38a,
	EarringsSingleSmallHoop = 0x388,
	EarringsSingleStud = 900,
	EarringsSmallHoops = 0x389,
	EarringsStuds = 0x385,
	GlassesCostume = 0x2be,
	// GlassesFirst = 700,
	GlassesGlasses = 700,
	// GlassesLast = 0x2be,
	GlassesSunglasses = 0x2bd,
	GlovesFingerless = 600,
	// GlovesFirst = 600,
	GlovesFullFingered = 0x259,
	// GlovesLast = 0x259,
	HatBaseballCap = 0x1f6,
	HatBeanie = 500,
	HatBearskin = 0x1fc,
	HatBrimmed = 0x1f8,
	HatCostume = 0x1fb,
	HatFez = 0x1f9,
	// HatFirst = 500,
	HatFlatCap = 0x1f5,
	HatHeadwrap = 0x1fa,
	HatHelmet = 0x1fd,
	// HatLast = 0x1fd,
	HatPeakCap = 0x1f7,
	// RingFirst = 0x3e8,
	RingLast = 0x3ea,
	RingLeft = 0x3e9,
	RingRight = 0x3e8,
	ShirtCoat = 210,
	// ShirtFirst = 200,
	ShirtHoodie = 0xd0,
	ShirtJacket = 0xd1,
	// ShirtLast = 210,
	ShirtLongSleeveShirt = 0xce,
	ShirtLongSleeveTee = 0xcc,
	ShirtPolo = 0xcb,
	ShirtShortSleeveShirt = 0xcd,
	ShirtSportsTee = 200,
	ShirtSweater = 0xcf,
	ShirtTee = 0xc9,
	ShirtVest = 0xca,
	ShoesCostume = 0x197,
	// ShoesFirst = 400,
	ShoesFormal = 0x193,
	ShoesHeels = 0x191,
	ShoesHighBoots = 0x196,
	// ShoesLast = 0x197,
	ShoesPumps = 0x192,
	ShoesSandals = 400,
	ShoesShortBoots = 0x195,
	ShoesTrainers = 0x194,
	TrousersCargo = 0x131,
	// TrousersFirst = 300,
	TrousersHotpants = 300,
	TrousersJeans = 0x132,
	TrousersKilt = 0x134,
	// TrousersLast = 0x135,
	TrousersLeggings = 0x12f,
	TrousersLongShorts = 0x12e,
	TrousersLongSkirt = 0x135,
	TrousersShorts = 0x12d,
	TrousersShortSkirt = 0x133,
	TrousersTrousers = 0x130,
	WristwearBands = 0x322,
	WristwearBracelet = 800,
	// WristwearFirst = 800,
	// WristwearLast = 0x323,
	WristwearSweatbands = 0x323,
	WristwearWatch = 0x321,
}

#[derive(Debug, Serialize)]
enum BinaryAssetType {
	Component = 1,
	Texture = 2,
	ShapeOverride = 3,
	Animation = 4,
	ShapeOverridePost = 5,
}

#[derive(Debug, Serialize)]
#[binrw]
#[brw(repr = u8)]
enum SkeletonVersion {
	Nxe = 1,
	Natal,
	NxeAndNatal,
}

#[derive(Debug, Serialize)]
#[binrw]
#[brw(repr = u8)]
enum AssetGender {
	Male = 1,
	Female,
	Both,
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct SvodVolumeDescriptor {
	size: u8,
	block_cache_element_count: u8,
	worker_thread_processor: u8,
	worker_thread_priority: u8,
	root_hash: [u8; 0x14],
	flags: u8,
	/// Encoded as an int24
	#[br(parse_with = binrw::helpers::read_u24)]
	#[bw(write_with = binrw::helpers::write_u24)]
	data_block_count: u32,
	/// Encoded as an int24
	#[br(parse_with = binrw::helpers::read_u24)]
	#[bw(write_with = binrw::helpers::write_u24)]
	data_block_offset: u32,
	reserved: [u8; 5],
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct AvatarAssetInformation {
	subcategory: AssetSubcategory,
	#[brw(little)]
	colorizable: u32,
	guid: [u8; 0x10],
	skeleton_version: SkeletonVersion,
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct MediaInformation {
	series_id: [u8; 0x10],
	season_id: [u8; 0x10],
	season_number: u16,
	episode_number: u16,
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct InstallerProgressCache {
	resume_state: OnlineContentResumeState,
	current_file_index: u32,
	current_file_offset: u64,
	bytes_processed: u64,
	timestamp_high: u32,
	timestamp_low: u32,
	#[br(count = 0)]
	cab_resume_data: Vec<u8>,
}

#[derive(Debug, Serialize)]
#[binrw]
pub struct FullInstallerMeta {
	installer_base_version: Version,
	installer_version: Version,
}

#[derive(Debug, Serialize, Variantly)]
#[binrw]
#[br(import(installer_type: InstallerType))]
pub enum InstallerMeta {
	#[br(pre_assert(installer_type.has_full_installer_meta()))]
	FullInstaller(FullInstallerMeta),
	#[br(pre_assert(installer_type.has_installer_progress_cache()))]
	InstallerProgressCache(InstallerProgressCache),
}

#[derive(Debug, Serialize, Clone, Copy)]
#[binrw]
#[brw(repr = u16)]
enum LicenseType {
	Unused = 0x0000,
	Unrestricted = 0xFFFF,
	ConsoleProfileLicense = 0x0009,
	WindowsProfileLicense = 0x0003,
	ConsoleLicense = 0xF000,
	MediaFlags = 0xE000,
	KeyVaultPrivileges = 0xD000,
	HyperVisorFlags = 0xC000,
	UserPrivileges = 0xB000,
}

impl Default for LicenseType {
	fn default() -> Self {
		Self::Unused
	}
}

#[derive(Default, Debug, Serialize)]
#[binrw]
pub struct LicenseEntry {
	ty: LicenseType,
	data: [u8; 6],
	bits: u32,
	flags: u32,
}

#[derive(Debug, Serialize)]
#[binrw]
#[br(import(content_type: ContentType))]
pub enum ContentMetadata {
	#[br(pre_assert(content_type == ContentType::AvatarItem))]
	AvatarItem(AvatarAssetInformation),

	#[br(pre_assert(content_type == ContentType::Video))]
	Video(MediaInformation),
}

#[derive(Debug, Serialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[binrw]
#[brw(repr = u32)]
pub enum ContentType {
	ArcadeGame = 0xD0000,
	AvatarAssetPack = 0x8000,
	AvatarItem = 0x9000,
	CacheFile = 0x40000,
	CommunityGame = 0x2000000,
	GameDemo = 0x80000,
	GameOnDemand = 0x7000,
	GamerPicture = 0x20000,
	GamerTitle = 0xA0000,
	GameTrailer = 0xC0000,
	GameVideo = 0x400000,
	InstalledGame = 0x4000,
	Installer = 0xB0000,
	IPTVPauseBuffer = 0x2000,
	LicenseStore = 0xF0000,
	MarketplaceContent = 2,
	Movie = 0x100000,
	MusicVideo = 0x300000,
	PodcastVideo = 0x500000,
	Profile = 0x10000,
	Publisher = 3,
	SavedGame = 1,
	StorageDownload = 0x50000,
	Theme = 0x30000,
	Video = 0x200000,
	ViralVideo = 0x600000,
	XboxDownload = 0x70000,
	XboxOriginalGame = 0x5000,
	XboxSavedGame = 0x60000,
	Xbox360Title = 0x1000,
	XNA = 0xE0000,
}

impl ContentType {
	pub fn has_content_metadata(&self) -> bool {
		matches!(self, ContentType::AvatarItem | ContentType::Video)
	}
}

#[derive(Debug, Serialize, Copy, Clone)]
#[binrw]
#[brw(repr = u32)]
pub enum InstallerType {
	None = 0,
	SystemUpdate = 0x53555044,
	TitleUpdate = 0x54555044,
	SystemUpdateProgressCache = 0x50245355,
	TitleUpdateProgressCache = 0x50245455,
	TitleContentProgressCache = 0x50245443,
}

impl InstallerType {
	pub fn has_full_installer_meta(&self) -> bool {
		matches!(self, InstallerType::SystemUpdate | InstallerType::TitleUpdate)
	}

	pub fn has_installer_progress_cache(&self) -> bool {
		matches!(
			self,
			InstallerType::SystemUpdateProgressCache
				| InstallerType::TitleUpdateProgressCache
				| Self::TitleContentProgressCache
		)
	}
}

#[derive(Debug, Serialize, Copy, Clone)]
#[binrw]
#[br(map = |input: u32| Self::from(input))]
#[bw(map = |this: &Self| u32::from(*this))]
pub struct Version {
	major: u16,
	minor: u16,
	build: u16,
	revision: u16,
}

impl From<u32> for Version {
	fn from(input: u32) -> Self {
		Version {
			major: ((input & 0xF000_0000) >> 28) as u16,
			minor: ((input & 0x0F00_0000) >> 24) as u16,
			build: ((input & 0x00FF_FF00) >> 8) as u16,
			revision: (input & 0xFF) as u16,
		}
	}
}

impl From<Version> for u32 {
	fn from(value: Version) -> Self {
		let Version { major, minor, build, revision } = value;
		let major = major as u32;
		let minor = minor as u32;
		let build = build as u32;
		let revision = revision as u32;

		(major << 28) | (minor << 24) | (build << 8) | revision
	}
}

#[derive(Debug, Serialize, Copy, Clone)]
#[binrw]
#[brw(repr = u32)]
enum OnlineContentResumeState {
	FileHeadersNotReady = 0x46494C48,
	NewFolder = 0x666F6C64,
	NewFolderResumeAttempt1 = 0x666F6C31,
	NewFolderResumeAttempt2 = 0x666F6C32,
	NewFolderResumeAttemptUnknown = 0x666F6C3F,
	NewFolderResumeAttemptSpecific = 0x666F6C40,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub enum XContentFlags {
	MetadataIsPEC = 1,
	MetadataSkipRead = 2,
	MetadataDontFreeThumbnails = 4,
}

#[derive(Debug, Serialize, PartialEq, Eq, Copy, Clone)]
#[binrw]
#[brw(repr = u32)]
pub enum FileSystemKind {
	Stfs = 0,
	Svod,
	Fatx,
}

#[derive(Debug, Serialize, Variantly)]
#[binrw]
#[br(import(fs_kind: FileSystemKind))]
pub enum FileSystem {
	#[br(pre_assert(fs_kind == FileSystemKind::Stfs))]
	Stfs(StfsVolumeDescriptor),

	#[br(pre_assert(fs_kind == FileSystemKind::Svod))]
	Svod(SvodVolumeDescriptor),

	#[br(pre_assert(fs_kind == FileSystemKind::Fatx))]
	Fatx,
}

impl Default for FileSystem {
	fn default() -> Self {
		FileSystem::Stfs(StfsVolumeDescriptor::default())
	}
}
