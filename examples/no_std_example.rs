// This example demonstrates using cv-rusty in a no_std environment
// Note: This is a simulated embedded environment that still uses std for demonstration purposes
// In a real embedded system, you would compile with --no-default-features

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use cv_rusty::Matrix3;

fn main() {
    println!("CV Rusty - no_std Example\n");
    println!("Demonstrating core functionality without file I/O\n");

    // Create a small 8x8 RGB image
    let mut image = Matrix3::zeros(8, 8);
    println!(
        "Created {}x{} image (zero-initialized)",
        image.width(),
        image.height()
    );

    // Draw a red diagonal line
    for i in 0..8 {
        image.set_pixel(i, i, 255, 0, 0);
    }
    println!("Drew red diagonal line");

    // Draw a green border
    for x in 0..8 {
        image.set_pixel(x, 0, 0, 255, 0);
        image.set_pixel(x, 7, 0, 255, 0);
    }
    for y in 0..8 {
        image.set_pixel(0, y, 0, 255, 0);
        image.set_pixel(7, y, 0, 255, 0);
    }
    println!("Drew green border");

    // Fill center with blue
    for y in 2..6 {
        for x in 2..6 {
            if x != y {
                // Skip diagonal (already red)
                image.set_pixel(x, y, 0, 0, 255);
            }
        }
    }
    println!("Filled center with blue\n");

    // Display the image as ASCII art
    println!("Image visualization (approximated):");
    println!("G = Green, R = Red, B = Blue, K = Black\n");

    for y in 0..8 {
        for x in 0..8 {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let c = if r > 200 && g < 50 && b < 50 {
                    'R' // Red
                } else if g > 200 && r < 50 && b < 50 {
                    'G' // Green
                } else if b > 200 && r < 50 && g < 50 {
                    'B' // Blue
                } else {
                    'K' // Black
                };
                print!("{} ", c);
            }
        }
        println!();
    }

    println!("\n✓ All operations completed without std library dependencies!");
    println!("  (Matrix3 only requires alloc for Vec)");

    // Demonstrate direct data access
    println!("\nDirect data access:");
    println!("  Total bytes: {}", image.data().len());
    println!("  Expected: {} bytes (8 × 8 × 3)", 8 * 8 * 3);

    // In an embedded system, you could now:
    // - Send this data over SPI/I2C to a display
    // - Process it with SIMD instructions
    // - Stream it over a network interface
    // - Store it in flash memory
    println!("\nIn a real embedded system, this image data could be:");
    println!("  • Sent to an LCD/OLED display via SPI");
    println!("  • Processed with hardware accelerators");
    println!("  • Streamed over a camera interface");
    println!("  • Stored in external flash memory");
}
