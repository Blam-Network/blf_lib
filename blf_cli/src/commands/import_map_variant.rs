use std::fs::{create_dir_all, exists, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Arc;
use blf_lib::blam::common::cache::cache_files::s_cache_file_header_v11;
use blf_lib::blf::chunks::{find_chunk, find_chunk_in_file};
use blf_lib::blf::s_blf_header;
use blf_lib::blf::versions::halo3::k_title_halo3;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_start_of_file;
use crate::console::console_task;
use memmap2::MmapOptions;
use stfs::vfs::VfsPath;
use crate::io::{build_path, get_directories_in_folder};
use crate::title_storage::halo3::v12070_08_09_05_2031_halo3_ship::k_build_string_halo3_ship_12070;

pub fn import_map_variant(
    config_path: String,
    map_variant_file_path: String,
    title: String,
    version: String,
) {
    let mut task = console_task::start(String::from("Importing Map Variant"));

    // 1. Get the BLF file.
    // We might be handling a STFS file...
    if !exists(&map_variant_file_path).unwrap() {
        task.fail(format!("Map variant not found at: {map_variant_file_path}"));
    }
    // let mut map_variant_file = File::open(&map_variant_file_path).unwrap();
    // let mut file_data = Vec::<u8>::new();
    // let mut blf_data = Vec::<u8>::new();
    // map_variant_file.read_to_end(&mut file_data).unwrap();
    // map_variant_file.seek(SeekFrom::Start(0));
    //
    // let is_blf_file = find_chunk::<s_blf_chunk_start_of_file>(&file_data).is_ok();
    //
    // if is_blf_file {
    //     blf_data = file_data.clone();
    // }
    //
    // if !is_blf_file {
        // try to read it as an STFS file.
        let file = File::open(map_variant_file_path).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

        let package = xcontent::XContentPackage::try_from(&mmap[..]).unwrap();

        let mut path: VfsPath = package.to_vfs_path(Arc::new(mmap));
    let mut foo = path.open_file().unwrap();
    let mut vector = Vec::<u8>::new();
    foo.read_to_end(&mut vector).unwrap();
    println!("Read blf {}", vector.len());

        // for file in path.walk_dir().unwrap() {
        //     let file = file.unwrap();
        //     // let meta = file.metadata().unwrap();
        //     if file.as_str().chars().filter(|c| *c == '/').count() == 1 {
        //         // let created: DateTime<Utc> = meta.created.unwrap().into();
        //         // let accessed: DateTime<Utc> = meta.accessed.unwrap().into();
        //
        //         println!(
        //             "{} {}b",
        //             if file.is_file().unwrap() { "f" } else { "d" },
        //             // meta.len,
        //             // created.format("%Y/%m/%d"),
        //             // accessed.format("%Y/%m/%d"),
        //             file.filename()
        //         );
        //     }
        // }

        return;

        let console_file = xcontent::XContentPackage::try_from(&mmap[..]);
        if console_file.is_err() {
            task.fail(format!("Unrecognized map variant file type."));
            panic!();
        }

        let package = console_file.unwrap();
        let path = package.to_vfs_path(Arc::new(mmap));
        // let foo = path.filename();
        // let bar = path.root();

        for file in path.walk_dir().unwrap() {
            let file = file.unwrap();
            let meta = file.metadata().unwrap();
            if file.as_str().chars().filter(|c| *c == '/').count() == 1 {
                // let created: DateTime<Utc> = meta.created.unwrap().into();
                // let accessed: DateTime<Utc> = meta.accessed.unwrap().into();

                println!(
                    "{} {}b {}",
                    if file.is_file().unwrap() { "f" } else { "d" },
                    meta.len,
                    // created.format("%Y/%m/%d"),
                    // accessed.format("%Y/%m/%d"),
                    file.filename()
                );
            }
        }

    //     let path = path.join("/sandbox.map").unwrap();
    //     if !path.exists().unwrap() {
    //         task.fail(format!("No map variant was found in the xbox package."));
    //         panic!();
    //     }
    //
    //     let mut sandbox_file = path.join("/sandbox.map").unwrap().open_file().unwrap();
    //     sandbox_file.read_to_end(&mut blf_data).unwrap();
    // }
    //
    // let foo = blf_data.len();
    // println!("{foo}");


    // if version == k_build_string_halo3_ship_12070 && title == k_title_halo3 {
    //     let cache_file = s_cache_file_header_v11::read(map_file_path);
    //     if cache_file.is_err() {
    //         task.fail(cache_file.unwrap_err());
    //         return;
    //     }
    //
    //     let cache_file = cache_file.unwrap();
    //
    //     let hopper_directories = get_directories_in_folder(&config_path).unwrap_or_else(|err|{
    //         task.fail(err);
    //         panic!()
    //     });
    //
    //     for hopper_directory in hopper_directories {
    //         let output_folder_path = build_path(vec![
    //             &config_path,
    //             &hopper_directory,
    //             &"rsa_signatures".to_string(),
    //         ]);
    //
    //         let output_file_path = build_path(vec![
    //             &output_folder_path,
    //             &cache_file.map_name.get_string(),
    //         ]);
    //
    //         create_dir_all(&output_folder_path).unwrap();
    //
    //         let mut output_file = File::create(output_file_path).unwrap();
    //         output_file.write_all(cache_file.rsa_signature.get()).unwrap();
    //     }
    // } else {
    //     task.add_error("Unsupported title and version.".to_string());
    // }

    task.complete();
}