//! Example demonstrating the drawing functionality.
//!
//! This example creates an image and draws various rectangles and circles on it.

use cv_rusty::{draw_circle, draw_rectangle, write_png, Color, Matrix3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a white canvas
    let mut image = Matrix3::zeros(600, 800);

    // Fill with white background
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }

    println!(
        "Drawing shapes on {}x{} canvas...",
        image.width(),
        image.height()
    );

    // Draw a filled red rectangle with black border (no rotation)
    draw_rectangle(
        &mut image,
        200.0,
        150.0, // position (x, y)
        150.0,
        100.0,                       // width, height
        0.0,                         // rotation in degrees
        3,                           // stroke width
        Some(Color::rgb(0, 0, 0)),   // stroke color (black)
        Some(Color::rgb(255, 0, 0)), // fill color (red)
    );
    println!("  ✓ Drew red rectangle at (200, 150)");

    // Draw a filled green rectangle with blue border (rotated 30 degrees)
    draw_rectangle(
        &mut image,
        550.0,
        150.0, // position (x, y)
        120.0,
        80.0,                        // width, height
        30.0,                        // rotation in degrees
        2,                           // stroke width
        Some(Color::rgb(0, 0, 255)), // stroke color (blue)
        Some(Color::rgb(0, 255, 0)), // fill color (green)
    );
    println!("  ✓ Drew rotated green rectangle at (550, 150)");

    // Draw a filled blue circle with white border
    draw_circle(
        &mut image,
        200.0,
        380.0,                           // position (x, y)
        60.0,                            // radius
        4,                               // stroke width
        Some(Color::rgb(255, 255, 255)), // stroke color (white)
        Some(Color::rgb(0, 0, 255)),     // fill color (blue)
    );
    println!("  ✓ Drew blue circle at (200, 380)");

    // Draw a filled yellow circle with black border
    draw_circle(
        &mut image,
        400.0,
        380.0,                         // position (x, y)
        50.0,                          // radius
        3,                             // stroke width
        Some(Color::rgb(0, 0, 0)),     // stroke color (black)
        Some(Color::rgb(255, 255, 0)), // fill color (yellow)
    );
    println!("  ✓ Drew yellow circle at (400, 380)");

    // Draw an unfilled circle (outline only)
    draw_circle(
        &mut image,
        600.0,
        380.0,                         // position (x, y)
        55.0,                          // radius
        5,                             // stroke width
        Some(Color::rgb(255, 0, 255)), // stroke color (magenta)
        None,                          // no fill
    );
    println!("  ✓ Drew magenta circle outline at (600, 380)");

    // Draw an unfilled rectangle (outline only, rotated 45 degrees)
    draw_rectangle(
        &mut image,
        400.0,
        500.0, // position (x, y)
        100.0,
        100.0,                         // width, height (square)
        45.0,                          // rotation in degrees
        4,                             // stroke width
        Some(Color::rgb(128, 0, 128)), // stroke color (purple)
        None,                          // no fill
    );
    println!("  ✓ Drew purple square outline at (400, 500)");

    // Draw overlapping semi-transparent-looking shapes
    // (Note: This is just RGB, not true alpha blending, but we can layer shapes)
    draw_rectangle(
        &mut image,
        150.0,
        500.0, // position (x, y)
        80.0,
        120.0,                         // width, height
        15.0,                          // rotation in degrees
        2,                             // stroke width
        Some(Color::rgb(0, 0, 0)),     // stroke color (black)
        Some(Color::rgb(255, 128, 0)), // fill color (orange)
    );
    println!("  ✓ Drew orange rectangle at (150, 500)");

    draw_circle(
        &mut image,
        170.0,
        480.0,                         // position (x, y)
        40.0,                          // radius
        2,                             // stroke width
        Some(Color::rgb(0, 0, 0)),     // stroke color (black)
        Some(Color::rgb(0, 255, 255)), // fill color (cyan)
    );
    println!("  ✓ Drew cyan circle at (170, 480)");

    // Save the result
    let output_path = "drawing_output.png";
    write_png(&image, output_path)?;
    println!("\n✓ Saved result to {}", output_path);
    println!("  Open the file to see the drawn shapes!");

    Ok(())
}
