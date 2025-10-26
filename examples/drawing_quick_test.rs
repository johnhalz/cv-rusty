//! Quick test example for drawing functionality.
//!
//! This is a minimal example to quickly verify drawing works.

use cv_rusty::{draw_circle, draw_rectangle, write_png, Color, Matrix3, Stroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a 400x300 white canvas
    let mut image = Matrix3::zeros(300, 400);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }

    // Draw a blue rectangle
    draw_rectangle(
        &mut image,
        100.0,
        75.0,
        80.0,
        50.0,
        0.0,
        Some(Stroke::new(2, Color::rgb(0, 0, 0))),
        Some(Color::rgb(0, 0, 255)),
    );

    // Draw a red circle
    draw_circle(
        &mut image,
        200.0,
        150.0,
        40.0,
        Some(Stroke::new(2, Color::rgb(0, 0, 0))),
        Some(Color::rgb(255, 0, 0)),
    );

    // Draw a rotated green rectangle
    draw_rectangle(
        &mut image,
        300.0,
        225.0,
        60.0,
        40.0,
        45.0,
        Some(Stroke::new(2, Color::rgb(0, 0, 0))),
        Some(Color::rgb(0, 255, 0)),
    );

    write_png(&image, "quick_test.png")?;
    println!("✓ Quick test passed! Saved to quick_test.png");

    Ok(())
}
