//! Example demonstrating hex color parsing for drawing.
//!
//! This example shows how to use hex color strings to draw shapes.

use cv_rusty::{draw_circle, draw_rectangle, write_png, Color, Matrix3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a white canvas
    let mut image = Matrix3::zeros(600, 800);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }

    println!(
        "Drawing shapes with hex colors on {}x{} canvas...",
        image.width(),
        image.height()
    );

    // Draw rectangles with hex colors (6-digit format)
    draw_rectangle(
        &mut image,
        150.0,
        150.0,
        120.0,
        80.0,
        0.0,
        3,
        Some(Color::from_hex("#000000")?), // Black border
        Some(Color::from_hex("#FF5733")?), // Orange fill
    );
    println!("  ✓ Drew orange rectangle (#FF5733)");

    draw_rectangle(
        &mut image,
        400.0,
        150.0,
        120.0,
        80.0,
        15.0, // Rotated
        3,
        Some(Color::from_hex("#2C3E50")?), // Dark blue-gray border
        Some(Color::from_hex("#3498DB")?), // Light blue fill
    );
    println!("  ✓ Drew blue rectangle (#3498DB)");

    draw_rectangle(
        &mut image,
        650.0,
        150.0,
        120.0,
        80.0,
        -15.0, // Rotated other way
        3,
        Some(Color::from_hex("#27AE60")?), // Dark green border
        Some(Color::from_hex("#2ECC71")?), // Light green fill
    );
    println!("  ✓ Drew green rectangle (#2ECC71)");

    // Draw circles with hex colors (3-digit format)
    draw_circle(
        &mut image,
        150.0,
        350.0,
        60.0,
        4,
        Some(Color::from_hex("#C00")?), // Red border (3-digit)
        Some(Color::from_hex("#F00")?), // Bright red fill (3-digit)
    );
    println!("  ✓ Drew red circle (#F00)");

    draw_circle(
        &mut image,
        400.0,
        350.0,
        60.0,
        4,
        Some(Color::from_hex("#808")?), // Purple border (3-digit)
        Some(Color::from_hex("#C0C")?), // Magenta fill (3-digit)
    );
    println!("  ✓ Drew magenta circle (#C0C)");

    draw_circle(
        &mut image,
        650.0,
        350.0,
        60.0,
        4,
        Some(Color::from_hex("#088")?), // Teal border (3-digit)
        Some(Color::from_hex("#0FF")?), // Cyan fill (3-digit)
    );
    println!("  ✓ Drew cyan circle (#0FF)");

    // Draw using FromStr trait (parse from string)
    let pink: Color = "#FF1493".parse()?;
    let dark_pink: Color = "#C71585".parse()?;

    draw_circle(
        &mut image,
        150.0,
        500.0,
        50.0,
        3,
        Some(dark_pink),
        Some(pink),
    );
    println!("  ✓ Drew pink circle using .parse() (#FF1493)");

    // Draw outline-only shapes with hex colors
    draw_rectangle(
        &mut image,
        400.0,
        500.0,
        100.0,
        100.0,
        45.0,
        5,
        Some(Color::from_hex("#9B59B6")?), // Purple
        None,
    );
    println!("  ✓ Drew purple square outline (#9B59B6)");

    draw_circle(
        &mut image,
        650.0,
        500.0,
        55.0,
        6,
        Some(Color::from_hex("#E67E22")?), // Orange
        None,
    );
    println!("  ✓ Drew orange circle outline (#E67E22)");

    // Draw with colors without '#' prefix
    draw_rectangle(
        &mut image,
        275.0,
        500.0,
        80.0,
        60.0,
        0.0,
        2,
        Some(Color::from_hex("000000")?), // Black (no hash)
        Some(Color::from_hex("FFD700")?), // Gold (no hash)
    );
    println!("  ✓ Drew gold rectangle (FFD700, no # prefix)");

    // Show common web colors
    let colors = [
        ("#E74C3C", "Red", 100.0, 50.0),
        ("#F39C12", "Orange", 200.0, 50.0),
        ("#F1C40F", "Yellow", 300.0, 50.0),
        ("#2ECC71", "Green", 400.0, 50.0),
        ("#3498DB", "Blue", 500.0, 50.0),
        ("#9B59B6", "Purple", 600.0, 50.0),
        ("#E91E63", "Pink", 700.0, 50.0),
    ];

    println!("\n  Drawing color palette:");
    for (hex, name, x, y) in colors.iter() {
        draw_circle(
            &mut image,
            *x,
            *y,
            20.0,
            2,
            Some(Color::from_hex("#34495E")?), // Dark gray border
            Some(Color::from_hex(hex)?),
        );
        println!("    • {} ({})", name, hex);
    }

    // Save the result
    let output_path = "drawing_hex_colors.png";
    write_png(&image, output_path)?;
    println!("\n✓ Saved result to {}", output_path);
    println!("  Open the file to see the colorful shapes!");

    Ok(())
}
