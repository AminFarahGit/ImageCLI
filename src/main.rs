use std::fs::{DirEntry};
use image::ImageReader;
use std::time::Instant;
use rayon::prelude::*;
use std::io::stdin;

enum FileType{
    JPG,
    PNG,
    JFIF,
    GIF,
}

impl FileType {
    fn extension(&self) -> &str {
        match self {
            FileType::JPG => "jpg",
            FileType::PNG => "png",
            FileType::JFIF => "jfif",
            FileType::GIF => "gif",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Enter the directory path of the images you would like converted: ");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Error Reading Input");

    if user_input.trim().is_empty() {
        println!("No directory path provided");
        return Ok(());
    }

    println!("Enter the file type you would like the images to be converted to (png, jpg, jfif, gif): ");
    let mut user_input_file_type = String::new();
    stdin().read_line(&mut user_input_file_type).expect("Error Reading Input");

    if user_input_file_type.trim().is_empty() || !["png", "jpg", "jfif", "gif"].contains(&user_input_file_type.trim()) {
        println!("No file type provided");
        return Ok(());
    }

    let file_type = match user_input_file_type.trim() {
        "png" => FileType::PNG,
        "jpg" => FileType::JPG,
        "jfif" => FileType::JFIF,
        "gif" => FileType::GIF,
        _ => unreachable!(),
    };

    let start = Instant::now();
    let files: Vec<DirEntry> = std::fs::read_dir(&user_input.trim())?.map(|f| f.unwrap()).collect();

    files.into_par_iter().for_each(|file| {
        if let Err(e) = convert_image_from_path(file, &file_type){
            eprintln!("Error converting image: {}", e);
        }
    });

    println!("took {:?}", start.elapsed());
    Ok(())

}

fn convert_image_from_path(file: DirEntry, file_type: &FileType) -> Result<(), Box<dyn std::error::Error>>{
    let path = file.path();
    let img = ImageReader::open(&path)?.decode()?;
    let out = path.with_extension(file_type.extension());

    img.save(&out)?;

    if out != path {
        std::fs::remove_file(&path)?;
    }

    Ok(())
}
