//! Example demonstrating the opacity feature.
//!
//! This example creates semi-transparent shapes that blend with each other
//! to demonstrate the new opacity functionality.

use cv_rusty::{draw_circle, draw_rectangle, write_png, Color, Matrix3, Stroke};

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
        "Drawing semi-transparent shapes on {}x{} canvas...",
        image.width(),
        image.height()
    );

    // Draw a solid blue rectangle as a base
    draw_rectangle(
        &mut image,
        100.0,
        100.0,
        200.0,
        150.0,
        0.0,
        None,
        Some(Color::rgb(0, 0, 255)),
    );
    println!("  ✓ Drew solid blue rectangle");

    // Draw a semi-transparent red rectangle overlapping the blue one
    draw_rectangle(
        &mut image,
        150.0,
        125.0,
        200.0,
        150.0,
        0.0,
        None,
        Some(Color::rgb_with_opacity(255, 0, 0, 0.5)),
    );
    println!("  ✓ Drew semi-transparent red rectangle (50% opacity)");

    // Draw three overlapping circles with different opacities
    // Red circle - 70% opacity
    draw_circle(
        &mut image,
        500.0,
        200.0,
        80.0,
        Some(Stroke::new(2, Color::rgb(128, 0, 0))),
        Some(Color::rgb_with_opacity(255, 0, 0, 0.7)),
    );
    println!("  ✓ Drew semi-transparent red circle (70% opacity)");

    // Green circle - 70% opacity
    draw_circle(
        &mut image,
        560.0,
        250.0,
        80.0,
        Some(Stroke::new(2, Color::rgb(0, 128, 0))),
        Some(Color::rgb_with_opacity(0, 255, 0, 0.7)),
    );
    println!("  ✓ Drew semi-transparent green circle (70% opacity)");

    // Blue circle - 70% opacity
    draw_circle(
        &mut image,
        530.0,
        280.0,
        80.0,
        Some(Stroke::new(2, Color::rgb(0, 0, 128))),
        Some(Color::rgb_with_opacity(0, 0, 255, 0.7)),
    );
    println!("  ✓ Drew semi-transparent blue circle (70% opacity)");

    // Draw gradient-like effect with multiple rectangles at different opacities
    let colors = [
        Color::rgb_with_opacity(255, 128, 0, 0.2), // 20% orange
        Color::rgb_with_opacity(255, 128, 0, 0.3), // 30% orange
        Color::rgb_with_opacity(255, 128, 0, 0.4), // 40% orange
        Color::rgb_with_opacity(255, 128, 0, 0.5), // 50% orange
        Color::rgb_with_opacity(255, 128, 0, 0.6), // 60% orange
    ];

    for (i, color) in colors.iter().enumerate() {
        draw_rectangle(
            &mut image,
            50.0 + (i as f32 * 30.0),
            400.0,
            80.0,
            150.0,
            0.0,
            Some(Stroke::new(1, Color::rgb(0, 0, 0))),
            Some(*color),
        );
    }
    println!("  ✓ Drew gradient effect with varying opacities");

    // Draw some text-like shapes with very low opacity for watermark effect
    draw_rectangle(
        &mut image,
        350.0,
        450.0,
        300.0,
        80.0,
        15.0,
        None,
        Some(Color::rgb_with_opacity(128, 128, 128, 0.15)),
    );
    println!("  ✓ Drew watermark-style rectangle (15% opacity)");

    // Draw fully transparent shape (should not be visible)
    draw_circle(
        &mut image,
        400.0,
        100.0,
        30.0,
        None,
        Some(Color::rgb_with_opacity(0, 0, 0, 0.0)),
    );
    println!("  ✓ Drew fully transparent circle (not visible)");

    // Save the result
    let output_path = "opacity_output.png";
    write_png(&image, output_path)?;
    println!("\n✓ Saved result to {}", output_path);
    println!("  Open the file to see the opacity effects!");
    println!("  Notice how the overlapping shapes blend together.");

    Ok(())
}
