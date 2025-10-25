//! Example demonstrating window display functionality.
//!
//! This example shows how to:
//! - Create and display color images (Matrix3)
//! - Create and display grayscale images (Matrix1)
//! - Use show_image_color and show_image functions
//! - Load and display images from files
//!
//! Run with: cargo run --example window_display_example --features window

#[cfg(feature = "window")]
use cv_rusty::{show_image, Matrix1, Matrix3};

#[cfg(feature = "window")]
fn main() {
    println!("Window Display Example");
    println!("======================\n");

    // Example 1: Display a simple gradient color image
    println!("Creating a color gradient image...");
    let width = 640;
    let height = 480;
    let mut color_image = Matrix3::zeros(width, height);

    // Create a color gradient (red to blue, top to bottom)
    for y in 0..height {
        for x in 0..width {
            let r = (255.0 * (1.0 - y as f32 / height as f32)) as u8;
            let g = (255.0 * x as f32 / width as f32) as u8;
            let b = (255.0 * y as f32 / height as f32) as u8;
            color_image.set_pixel(x, y, r, g, b);
        }
    }

    println!("Displaying color image. Press ESC or close the window to continue...");
    if let Err(e) = show_image("Color Gradient", &color_image) {
        eprintln!("Error displaying color image: {}", e);
    }

    // Example 2: Display a grayscale gradient image
    println!("\nCreating a grayscale gradient image...");
    let mut gray_image = Matrix1::zeros(width, height);

    // Create a circular gradient
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_radius = (center_x * center_x + center_y * center_y).sqrt();

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let intensity = (255.0 * (1.0 - distance / max_radius)) as u8;
            gray_image.set_pixel(x, y, intensity);
        }
    }

    println!("Displaying grayscale image. Press ESC or close the window to continue...");
    if let Err(e) = show_image("Grayscale Radial Gradient", &gray_image) {
        eprintln!("Error displaying grayscale image: {}", e);
    }

    // Example 3: Display a checkerboard pattern
    println!("\nCreating a checkerboard pattern...");
    let mut checkerboard = Matrix3::zeros(width, height);
    let square_size = 40;

    for y in 0..height {
        for x in 0..width {
            let is_white = ((x / square_size) + (y / square_size)) % 2 == 0;
            let color = if is_white { 255 } else { 0 };
            checkerboard.set_pixel(x, y, color, color, color);
        }
    }

    println!("Displaying checkerboard. Press ESC or close the window to continue...");
    if let Err(e) = show_image("Checkerboard", &checkerboard) {
        eprintln!("Error displaying checkerboard: {}", e);
    }

    // Example 4: If an image file exists, display it
    #[cfg(feature = "std")]
    {
        use cv_rusty::read_jpeg;
        use std::path::Path;

        if Path::new("test.jpg").exists() {
            println!("\nLoading and displaying test.jpg...");
            match read_jpeg("test.jpg") {
                Ok(image) => {
                    println!("Loaded image: {}x{}", image.width(), image.height());
                    println!("Press ESC or close the window to exit...");
                    if let Err(e) = show_image("test.jpg", &image) {
                        eprintln!("Error displaying image: {}", e);
                    }
                }
                Err(e) => eprintln!("Error loading test.jpg: {}", e),
            }
        } else {
            println!("\nNo test.jpg found. Skipping file display example.");
            println!("To test with a real image, place a JPEG file named 'test.jpg' in the current directory.");
        }
    }

    println!("\nExample complete!");
}

#[cfg(not(feature = "window"))]
fn main() {
    eprintln!("This example requires the 'window' feature to be enabled.");
    eprintln!("Run with: cargo run --example window_display_example --features window");
}
