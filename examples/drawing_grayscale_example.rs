//! Example demonstrating drawing on grayscale images.
//!
//! This example creates a grayscale image and draws various rectangles and circles on it.

use cv_rusty::{draw_circle, draw_rectangle, write_png, Color, Matrix1, Matrix3, Stroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a grayscale canvas (mid-gray background)
    let mut image = Matrix1::zeros(600, 800);

    // Fill with mid-gray background
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 200);
        }
    }

    println!(
        "Drawing shapes on {}x{} grayscale canvas...",
        image.width(),
        image.height()
    );

    // Draw a filled dark rectangle with white border (no rotation)
    draw_rectangle(
        &mut image,
        200.0,
        150.0, // position (x, y)
        150.0,
        100.0,                                  // width, height
        0.0,                                    // rotation in degrees
        Some(Stroke::new(3, Color::gray(255))), // stroke (3px white)
        Some(Color::gray(50)),                  // fill color (dark gray)
    );
    println!("  ✓ Drew dark rectangle at (200, 150)");

    // Draw a filled light rectangle with black border (rotated 30 degrees)
    draw_rectangle(
        &mut image,
        550.0,
        150.0, // position (x, y)
        120.0,
        80.0,                                 // width, height
        30.0,                                 // rotation in degrees
        Some(Stroke::new(2, Color::gray(0))), // stroke (2px black)
        Some(Color::gray(220)),               // fill color (light gray)
    );
    println!("  ✓ Drew rotated light rectangle at (550, 150)");

    // Draw a filled dark circle with white border
    draw_circle(
        &mut image,
        200.0,
        380.0,                                  // position (x, y)
        60.0,                                   // radius
        Some(Stroke::new(4, Color::gray(255))), // stroke (4px white)
        Some(Color::gray(80)),                  // fill color (dark gray)
    );
    println!("  ✓ Drew dark circle at (200, 380)");

    // Draw a filled light circle with black border
    draw_circle(
        &mut image,
        400.0,
        380.0,                                // position (x, y)
        50.0,                                 // radius
        Some(Stroke::new(3, Color::gray(0))), // stroke (3px black)
        Some(Color::gray(240)),               // fill color (light gray)
    );
    println!("  ✓ Drew light circle at (400, 380)");

    // Draw an unfilled circle (outline only)
    draw_circle(
        &mut image,
        600.0,
        380.0,                                  // position (x, y)
        55.0,                                   // radius
        Some(Stroke::new(5, Color::gray(100))), // stroke (5px medium gray)
        None,                                   // no fill
    );
    println!("  ✓ Drew circle outline at (600, 380)");

    // Draw an unfilled rectangle (outline only, rotated 45 degrees)
    draw_rectangle(
        &mut image,
        400.0,
        500.0, // position (x, y)
        100.0,
        100.0,                                 // width, height (square)
        45.0,                                  // rotation in degrees
        Some(Stroke::new(4, Color::gray(50))), // stroke (4px dark gray)
        None,                                  // no fill
    );
    println!("  ✓ Drew square outline at (400, 500)");

    // Draw overlapping shapes
    draw_rectangle(
        &mut image,
        150.0,
        500.0, // position (x, y)
        80.0,
        120.0,                                // width, height
        15.0,                                 // rotation in degrees
        Some(Stroke::new(2, Color::gray(0))), // stroke (2px black)
        Some(Color::gray(150)),               // fill color (medium gray)
    );
    println!("  ✓ Drew medium rectangle at (150, 500)");

    draw_circle(
        &mut image,
        170.0,
        480.0,                                  // position (x, y)
        40.0,                                   // radius
        Some(Stroke::new(2, Color::gray(255))), // stroke (2px white)
        Some(Color::gray(180)),                 // fill color (light-medium gray)
    );
    println!("  ✓ Drew light-medium circle at (170, 480)");

    // Convert to RGB for saving (PNG requires RGB or RGBA)
    let mut rgb_image = Matrix3::zeros(image.width(), image.height());
    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some(gray) = image.get_pixel(x, y) {
                rgb_image.set_pixel(x, y, gray, gray, gray);
            }
        }
    }

    // Save the result
    let output_path = "drawing_grayscale_output.png";
    write_png(&rgb_image, output_path)?;
    println!("\n✓ Saved result to {}", output_path);
    println!("  Open the file to see the drawn shapes!");

    Ok(())
}
