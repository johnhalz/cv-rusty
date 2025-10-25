//! Simple example showing basic image display usage.
//!
//! Run with: cargo run --example simple_show_image --features window

#[cfg(feature = "window")]
use cv_rusty::{show_image, Matrix3};

#[cfg(feature = "window")]
fn main() {
    // Create a simple test image
    let width = 400;
    let height = 300;
    let mut image = Matrix3::zeros(width, height);

    // Draw a simple pattern - red square in center
    for y in 100..200 {
        for x in 150..250 {
            image.set_pixel(x, y, 255, 0, 0); // Red
        }
    }

    // Draw blue border
    for x in 0..width {
        image.set_pixel(x, 0, 0, 0, 255); // Top border
        image.set_pixel(x, height - 1, 0, 0, 255); // Bottom border
    }
    for y in 0..height {
        image.set_pixel(0, y, 0, 0, 255); // Left border
        image.set_pixel(width - 1, y, 0, 0, 255); // Right border
    }

    // Display the image
    println!("Displaying image. Press ESC or close window to exit.");
    show_image("Simple Image", &image).expect("Failed to display image");
}

#[cfg(not(feature = "window"))]
fn main() {
    eprintln!("This example requires the 'window' feature.");
    eprintln!("Run with: cargo run --example simple_show_image --features window");
}
