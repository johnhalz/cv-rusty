//! Minimal test to verify window display works correctly.
//!
//! Run with: cargo run --example test_window --features window

#[cfg(feature = "window")]
use cv_rusty::{imshow_color, Matrix3};

#[cfg(feature = "window")]
fn main() {
    println!("Testing window display...");
    println!("Creating a test image with colored regions...");

    let width = 400;
    let height = 300;
    let mut image = Matrix3::zeros(width, height);

    // Fill with white background
    for y in 0..height {
        for x in 0..width {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }

    // Red square (top-left)
    for y in 50..150 {
        for x in 50..150 {
            image.set_pixel(x, y, 255, 0, 0);
        }
    }

    // Green square (top-right)
    for y in 50..150 {
        for x in 250..350 {
            image.set_pixel(x, y, 0, 255, 0);
        }
    }

    // Blue square (bottom-left)
    for y in 150..250 {
        for x in 50..150 {
            image.set_pixel(x, y, 0, 0, 255);
        }
    }

    // Yellow square (bottom-right)
    for y in 150..250 {
        for x in 250..350 {
            image.set_pixel(x, y, 255, 255, 0);
        }
    }

    println!("Displaying image...");
    println!("You should see:");
    println!("  - White background");
    println!("  - Red square (top-left)");
    println!("  - Green square (top-right)");
    println!("  - Blue square (bottom-left)");
    println!("  - Yellow square (bottom-right)");
    println!();
    println!("Press ESC or close the window to exit.");

    match imshow_color("Window Test - Should see 4 colored squares", &image) {
        Ok(_) => println!("\nWindow closed successfully!"),
        Err(e) => eprintln!("\nError displaying window: {}", e),
    }
}

#[cfg(not(feature = "window"))]
fn main() {
    eprintln!("This example requires the 'window' feature.");
    eprintln!("Run with: cargo run --example test_window --features window");
}
