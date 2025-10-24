//! Example demonstrating reading an image and converting it to different formats.
//!
//! This example reads an image file (JPEG or PNG), displays information about it,
//! and then saves it in both JPEG and PNG formats with different quality settings.
//!
//! Usage:
//!     cargo run --example image_conversion <input_file>
//!
//! Example:
//!     cargo run --example image_conversion photo.jpg

use cv_rusty::{read_jpeg, read_png, write_jpeg, write_png};
use std::env;
use std::path::Path;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} photo.jpg", args[0]);
        eprintln!("  {} image.png", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let path = Path::new(input_path);

    // Check if file exists
    if !path.exists() {
        eprintln!("Error: File '{}' does not exist", input_path);
        std::process::exit(1);
    }

    // Determine file type and read the image
    let image = match path.extension().and_then(|s| s.to_str()) {
        Some("jpg") | Some("jpeg") | Some("JPG") | Some("JPEG") => {
            println!("Reading JPEG file: {}", input_path);
            match read_jpeg(input_path) {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Error reading JPEG: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some("png") | Some("PNG") => {
            println!("Reading PNG file: {}", input_path);
            match read_png(input_path) {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Error reading PNG: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unsupported file format. Only JPEG and PNG are supported.");
            std::process::exit(1);
        }
    };

    // Display image information
    println!("\n✓ Image loaded successfully!");
    println!("  Dimensions: {}x{}", image.width(), image.height());
    println!("  Total pixels: {}", image.width() * image.height());
    println!("  Data size: {} bytes", image.data().len());

    // Get the base filename without extension
    let base_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    println!("\n--- Converting to different formats ---\n");

    // Save as high-quality JPEG
    let jpeg_high = format!("{}_high_quality.jpg", base_name);
    print!("Writing high quality JPEG (quality: 95)... ");
    match write_jpeg(&image, &jpeg_high, 95) {
        Ok(_) => println!("✓ {}", jpeg_high),
        Err(e) => eprintln!("✗ Failed: {}", e),
    }

    // Save as medium-quality JPEG
    let jpeg_medium = format!("{}_medium_quality.jpg", base_name);
    print!("Writing medium quality JPEG (quality: 75)... ");
    match write_jpeg(&image, &jpeg_medium, 75) {
        Ok(_) => println!("✓ {}", jpeg_medium),
        Err(e) => eprintln!("✗ Failed: {}", e),
    }

    // Save as low-quality JPEG
    let jpeg_low = format!("{}_low_quality.jpg", base_name);
    print!("Writing low quality JPEG (quality: 40)... ");
    match write_jpeg(&image, &jpeg_low, 40) {
        Ok(_) => println!("✓ {}", jpeg_low),
        Err(e) => eprintln!("✗ Failed: {}", e),
    }

    // Save as PNG (lossless)
    let png_output = format!("{}_converted.png", base_name);
    print!("Writing lossless PNG... ");
    match write_png(&image, &png_output) {
        Ok(_) => println!("✓ {}", png_output),
        Err(e) => eprintln!("✗ Failed: {}", e),
    }

    println!("\n--- Image Processing Complete ---");
    println!("\nYou can now compare the file sizes and quality:");
    println!("  ls -lh {}_*", base_name);
}
