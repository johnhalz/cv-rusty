//! Example demonstrating color space conversions
//!
//! This example shows how to:
//! - Convert RGB images to grayscale using different methods
//! - Convert between RGB, HSV, and HSL color spaces
//! - Work with Matrix1 (grayscale) and Matrix3 (RGB) types

use cv_rusty::{hsl_to_rgb, hsv_to_rgb, rgb_to_hsl, rgb_to_hsv, GrayscaleMethod, Matrix1, Matrix3};

fn main() {
    println!("=== Color Space Conversion Example ===\n");

    // Create a simple RGB image with different colored pixels
    let mut rgb_image = Matrix3::zeros(4, 4);

    // Set some interesting colors
    rgb_image.set_pixel(0, 0, 255, 0, 0); // Red
    rgb_image.set_pixel(1, 0, 0, 255, 0); // Green
    rgb_image.set_pixel(2, 0, 0, 0, 255); // Blue
    rgb_image.set_pixel(3, 0, 255, 255, 0); // Yellow

    rgb_image.set_pixel(0, 1, 255, 0, 255); // Magenta
    rgb_image.set_pixel(1, 1, 0, 255, 255); // Cyan
    rgb_image.set_pixel(2, 1, 255, 255, 255); // White
    rgb_image.set_pixel(3, 1, 128, 128, 128); // Gray

    rgb_image.set_pixel(0, 2, 192, 64, 64); // Dark red
    rgb_image.set_pixel(1, 2, 64, 192, 64); // Dark green
    rgb_image.set_pixel(2, 2, 64, 64, 192); // Dark blue
    rgb_image.set_pixel(3, 2, 255, 128, 0); // Orange

    println!("Original RGB image: {}", rgb_image);
    println!("Dimensions: {}x{}\n", rgb_image.width(), rgb_image.height());

    // === RGB to Grayscale Conversions ===
    println!("=== RGB to Grayscale Conversions ===\n");

    // Method 1: Luminosity (recommended - accounts for human perception)
    let gray_luminosity = rgb_image.to_grayscale();
    println!("Grayscale (Luminosity method): {}", gray_luminosity);
    println!("  Formula: 0.299*R + 0.587*G + 0.114*B");
    print_grayscale_samples(&gray_luminosity);

    // Method 2: Average
    let gray_average = rgb_image.to_grayscale_average();
    println!("\nGrayscale (Average method): {}", gray_average);
    println!("  Formula: (R + G + B) / 3");
    print_grayscale_samples(&gray_average);

    // Method 3: Lightness
    let gray_lightness = rgb_image.to_grayscale_lightness();
    println!("\nGrayscale (Lightness method): {}", gray_lightness);
    println!("  Formula: (max(R,G,B) + min(R,G,B)) / 2");
    print_grayscale_samples(&gray_lightness);

    // Using the method parameter
    let gray_custom = rgb_image.to_grayscale_with_method(GrayscaleMethod::Luminosity);
    println!("\nGrayscale (Custom method selection): {}", gray_custom);

    // === RGB to HSV Conversions ===
    println!("\n=== RGB to HSV Conversions ===\n");

    let colors = vec![
        ("Red", 255, 0, 0),
        ("Green", 0, 255, 0),
        ("Blue", 0, 0, 255),
        ("Yellow", 255, 255, 0),
        ("Cyan", 0, 255, 255),
        ("Magenta", 255, 0, 255),
    ];

    for (name, r, g, b) in &colors {
        let (h, s, v) = rgb_to_hsv(*r, *g, *b);
        println!(
            "{:8} RGB({:3}, {:3}, {:3}) -> HSV({:6.1}°, {:.3}, {:.3})",
            name, r, g, b, h, s, v
        );
    }

    // === HSV to RGB Conversions ===
    println!("\n=== HSV to RGB Conversions ===\n");

    let hsv_colors = vec![
        ("Red", 0.0, 1.0, 1.0),
        ("Green", 120.0, 1.0, 1.0),
        ("Blue", 240.0, 1.0, 1.0),
        ("Half bright green", 120.0, 1.0, 0.5),
        ("Desaturated red", 0.0, 0.5, 1.0),
    ];

    for (name, h, s, v) in &hsv_colors {
        let (r, g, b) = hsv_to_rgb(*h, *s, *v);
        println!(
            "{:18} HSV({:6.1}°, {:.1}, {:.1}) -> RGB({:3}, {:3}, {:3})",
            name, h, s, v, r, g, b
        );
    }

    // === RGB to HSL Conversions ===
    println!("\n=== RGB to HSL Conversions ===\n");

    for (name, r, g, b) in &colors {
        let (h, s, l) = rgb_to_hsl(*r, *g, *b);
        println!(
            "{:8} RGB({:3}, {:3}, {:3}) -> HSL({:6.1}°, {:.3}, {:.3})",
            name, r, g, b, h, s, l
        );
    }

    // === HSL to RGB Conversions ===
    println!("\n=== HSL to RGB Conversions ===\n");

    let hsl_colors = vec![
        ("Red", 0.0, 1.0, 0.5),
        ("Green", 120.0, 1.0, 0.5),
        ("Blue", 240.0, 1.0, 0.5),
        ("Light red", 0.0, 1.0, 0.75),
        ("Dark green", 120.0, 1.0, 0.25),
    ];

    for (name, h, s, l) in &hsl_colors {
        let (r, g, b) = hsl_to_rgb(*h, *s, *l);
        println!(
            "{:12} HSL({:6.1}°, {:.1}, {:.2}) -> RGB({:3}, {:3}, {:3})",
            name, h, s, l, r, g, b
        );
    }

    // === Roundtrip Conversion Test ===
    println!("\n=== Roundtrip Conversion Test ===\n");

    let test_rgb = (192, 64, 128);
    println!(
        "Original RGB: ({}, {}, {})",
        test_rgb.0, test_rgb.1, test_rgb.2
    );

    // RGB -> HSV -> RGB
    let (h, s, v) = rgb_to_hsv(test_rgb.0, test_rgb.1, test_rgb.2);
    let (r2, g2, b2) = hsv_to_rgb(h, s, v);
    println!(
        "  RGB -> HSV -> RGB: ({}, {}, {}) [HSV: {:.1}°, {:.3}, {:.3}]",
        r2, g2, b2, h, s, v
    );

    // RGB -> HSL -> RGB
    let (h, s, l) = rgb_to_hsl(test_rgb.0, test_rgb.1, test_rgb.2);
    let (r3, g3, b3) = hsl_to_rgb(h, s, l);
    println!(
        "  RGB -> HSL -> RGB: ({}, {}, {}) [HSL: {:.1}°, {:.3}, {:.3}]",
        r3, g3, b3, h, s, l
    );

    // === Working with Grayscale Images ===
    println!("\n=== Working with Grayscale Images ===\n");

    let mut gray = Matrix1::zeros(3, 3);
    println!("Created grayscale image: {}", gray);

    // Set a gradient
    for y in 0..3 {
        for x in 0..3 {
            let value = (x * 85 + y * 85) as u8;
            gray.set_pixel(x, y, value);
        }
    }

    println!("\nGradient values:");
    for y in 0..3 {
        print!("  ");
        for x in 0..3 {
            if let Some(val) = gray.get_pixel(x, y) {
                print!("{:3} ", val);
            }
        }
        println!();
    }

    println!("\n=== Example Complete ===");
}

fn print_grayscale_samples(gray: &Matrix1) {
    println!("  Sample values:");
    let samples = vec![
        (0, 0, "Red"),
        (1, 0, "Green"),
        (2, 0, "Blue"),
        (3, 0, "Yellow"),
    ];

    for (x, y, name) in samples {
        if let Some(val) = gray.get_pixel(x, y) {
            println!("    {:8} -> {}", name, val);
        }
    }
}
