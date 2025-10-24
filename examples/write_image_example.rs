//! Example demonstrating how to write JPEG and PNG images using cv-rusty.
//!
//! This example creates a simple gradient image and saves it in both JPEG and PNG formats.

use cv_rusty::{write_jpeg, write_png, Matrix3};

fn main() {
    println!("Creating a gradient image...");

    // Create a 640x480 image with a colorful gradient
    let width = 640;
    let height = 480;
    let mut data = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        for x in 0..width {
            // Create a gradient effect
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;

            data.push(r);
            data.push(g);
            data.push(b);
        }
    }

    let image = Matrix3::new(width, height, data);
    println!("Created {}x{} image", image.width(), image.height());

    // Write JPEG with quality 90
    println!("Writing JPEG image...");
    match write_jpeg(&image, "gradient.jpg", 90) {
        Ok(_) => println!("✓ Successfully wrote gradient.jpg (quality: 90)"),
        Err(e) => eprintln!("✗ Failed to write JPEG: {}", e),
    }

    // Write JPEG with lower quality to show compression difference
    println!("Writing JPEG image with lower quality...");
    match write_jpeg(&image, "gradient_low_quality.jpg", 50) {
        Ok(_) => println!("✓ Successfully wrote gradient_low_quality.jpg (quality: 50)"),
        Err(e) => eprintln!("✗ Failed to write JPEG: {}", e),
    }

    // Write PNG (lossless)
    println!("Writing PNG image...");
    match write_png(&image, "gradient.png") {
        Ok(_) => println!("✓ Successfully wrote gradient.png (lossless)"),
        Err(e) => eprintln!("✗ Failed to write PNG: {}", e),
    }

    println!("\nExample complete! Check the current directory for the output images.");
}
