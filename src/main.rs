extern crate gdal;

use gdal::Dataset;
use gdal::DriverManager;
use gdal::raster::RasterCreationOption;
use std::path::Path;
use std::fs;
use rayon::prelude::*;
use std::env;




fn create_cog(filepath: &str, folder: &str) {
    println!("Creating COG for {}", filepath);
    // Path to your raster file
    let file_path = filepath;

    let basename = Path::new(file_path).file_stem().unwrap().to_str();

    let tif_path = format!("{}/{}.tif", folder, basename.unwrap());

    // Open the dataset
    let dataset = Dataset::open(file_path).expect("Failed to open dataset");

    let driver = DriverManager::get_driver_by_name("COG").expect("Failed to get driver");

    let creation_options = [
        RasterCreationOption {
            key: "COMPRESS",
            value: "LZW"
        },
        RasterCreationOption {
            key: "COMPRESS",
            value: "LZW"
        }
    ];

    let _cog = dataset
        .create_copy(
            &driver,
            tif_path,
            &creation_options
        );

    // Create the COG
    // let tif = driver
    //     // .create(
    //     //     "test.tif",
    //     //     dataset.raster_size().0 as isize,
    //     //     dataset.raster_size().1 as isize,
    //     //     dataset.raster_count() as isize);
    //     .create_with_band_type_with_options::<u8, _>(
    //         "test.tif",
    //         dataset.raster_size().0 as isize,
    //         dataset.raster_size().1 as isize,
    //         dataset.raster_count() as isize,
    //         &creation_options,
    //     );

    // // Print the raster size
    // println!("Raster size: {} x {}", raster_size.0, raster_size.1);
}

fn main() {
    
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    // let path = "/home/bjyberg/bioversity/RangelandData/historical";

    let start_time = std::time::Instant::now();

    let files: Vec<String> = fs::read_dir(path)
        .expect("Failed to read directory")
        .map(|entry| entry.unwrap().path().display().to_string())
        .collect();

    // let files: Vec<String> = fs::read_dir(path)
    // .expect("Failed to read directory")
    // .filter_map(|entry| {
    //     entry.ok()
    //         .map(|e| e.path().to_string_lossy().into_owned())
    // })
    // .collect();

    files.par_iter().for_each(|x| create_cog(x, path));

    let end_time = std::time::Instant::now();
    let total_time = end_time - start_time;
    println!("Total processing time: {:?}", total_time);
}