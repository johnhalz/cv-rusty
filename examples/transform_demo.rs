//! Transform demo - demonstrates resize, crop, and rotate operations
//!
//! This example shows how to use the image transformation operations
//! including resizing with different interpolation methods, cropping,
//! and rotating images.

use cv_rusty::{read_jpeg, write_jpeg, InterpolationMethod, Matrix3, Rotation, RotationAngle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CV Rusty Transform Demo ===\n");

    // Create a simple test image with a gradient pattern
    println!("Creating test image (200x150)...");
    let mut image = Matrix3::zeros(200, 150);
    for y in 0..150 {
        for x in 0..200 {
            let r = (x * 255 / 200) as u8;
            let g = (y * 255 / 150) as u8;
            let b = 128;
            image.set_pixel(x, y, r, g, b);
        }
    }
    println!("Original image: {}x{}", image.width(), image.height());

    // Test 1: Resize with Nearest Neighbor
    println!("\n--- Resize Operations ---");
    println!("Resizing to 100x75 (Nearest Neighbor)...");
    let resized_nn = image.resize(100, 75, InterpolationMethod::NearestNeighbor);
    println!("Result: {}x{}", resized_nn.width(), resized_nn.height());
    write_jpeg(&resized_nn, "output_resize_nn.jpg", 90)?;
    println!("Saved: output_resize_nn.jpg");

    // Test 2: Resize with Bilinear interpolation
    println!("\nResizing to 100x75 (Bilinear)...");
    let resized_bilinear = image.resize(100, 75, InterpolationMethod::Bilinear);
    println!(
        "Result: {}x{}",
        resized_bilinear.width(),
        resized_bilinear.height()
    );
    write_jpeg(&resized_bilinear, "output_resize_bilinear.jpg", 90)?;
    println!("Saved: output_resize_bilinear.jpg");

    // Test 3: Upscaling
    println!("\nUpscaling to 400x300 (Bilinear)...");
    let upscaled = image.resize(400, 300, InterpolationMethod::Bilinear);
    println!("Result: {}x{}", upscaled.width(), upscaled.height());
    write_jpeg(&upscaled, "output_upscaled.jpg", 90)?;
    println!("Saved: output_upscaled.jpg");

    // Test 4: Crop operation
    println!("\n--- Crop Operations ---");
    println!("Cropping region (50, 30) to (100x80)...");
    match image.crop(50, 30, 100, 80) {
        Some(cropped) => {
            println!("Result: {}x{}", cropped.width(), cropped.height());
            write_jpeg(&cropped, "output_cropped.jpg", 90)?;
            println!("Saved: output_cropped.jpg");
        }
        None => println!("Error: Invalid crop region"),
    }

    // Test 5: Center crop
    println!("\nCenter cropping to 100x100...");
    let center_x = (image.width() - 100) / 2;
    let center_y = (image.height() - 100) / 2;
    match image.crop(center_x, center_y, 100, 100) {
        Some(cropped) => {
            println!("Result: {}x{}", cropped.width(), cropped.height());
            write_jpeg(&cropped, "output_center_crop.jpg", 90)?;
            println!("Saved: output_center_crop.jpg");
        }
        None => println!("Error: Invalid crop region"),
    }

    // Test 6: Rotation operations
    println!("\n--- Rotation Operations ---");
    println!("Rotating 90 degrees clockwise...");
    let rotated_90 = image.rotate(RotationAngle::Rotate90);
    println!("Result: {}x{}", rotated_90.width(), rotated_90.height());
    write_jpeg(&rotated_90, "output_rotate_90.jpg", 90)?;
    println!("Saved: output_rotate_90.jpg");

    println!("\nRotating 180 degrees...");
    let rotated_180 = image.rotate(RotationAngle::Rotate180);
    println!("Result: {}x{}", rotated_180.width(), rotated_180.height());
    write_jpeg(&rotated_180, "output_rotate_180.jpg", 90)?;
    println!("Saved: output_rotate_180.jpg");

    println!("\nRotating 270 degrees clockwise...");
    let rotated_270 = image.rotate(RotationAngle::Rotate270);
    println!("Result: {}x{}", rotated_270.width(), rotated_270.height());
    write_jpeg(&rotated_270, "output_rotate_270.jpg", 90)?;
    println!("Saved: output_rotate_270.jpg");

    // Test 7: Custom rotation with arbitrary angles
    println!("\n--- Custom Rotation Operations ---");
    println!("Rotating 45 degrees (using Rotation::Degrees)...");
    let rotated_45 = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
    println!("Result: {}x{}", rotated_45.width(), rotated_45.height());
    write_jpeg(&rotated_45, "output_rotate_45deg.jpg", 90)?;
    println!("Saved: output_rotate_45deg.jpg");

    println!("\nRotating 30 degrees with nearest neighbor...");
    let rotated_30 = image.rotate_custom(
        Rotation::Degrees(30.0),
        InterpolationMethod::NearestNeighbor,
    );
    println!("Result: {}x{}", rotated_30.width(), rotated_30.height());
    write_jpeg(&rotated_30, "output_rotate_30deg_nn.jpg", 90)?;
    println!("Saved: output_rotate_30deg_nn.jpg");

    println!("\nRotating PI/6 radians (30 degrees using radians)...");
    let rotated_pi6 = image.rotate_custom(
        Rotation::Radians(std::f32::consts::PI / 6.0),
        InterpolationMethod::Bilinear,
    );
    println!("Result: {}x{}", rotated_pi6.width(), rotated_pi6.height());
    write_jpeg(&rotated_pi6, "output_rotate_pi6_rad.jpg", 90)?;
    println!("Saved: output_rotate_pi6_rad.jpg");

    println!("\nRotating -22.5 degrees (counter-clockwise)...");
    let rotated_neg = image.rotate_custom(Rotation::Degrees(-22.5), InterpolationMethod::Bilinear);
    println!("Result: {}x{}", rotated_neg.width(), rotated_neg.height());
    write_jpeg(&rotated_neg, "output_rotate_neg22.jpg", 90)?;
    println!("Saved: output_rotate_neg22.jpg");

    // Test 8: Combined operations
    println!("\n--- Combined Operations ---");
    println!("Crop -> Resize -> Rotate pipeline...");
    let combined = image
        .crop(25, 25, 150, 100)
        .expect("Crop failed")
        .resize(75, 50, InterpolationMethod::Bilinear)
        .rotate(RotationAngle::Rotate90);
    println!("Result: {}x{}", combined.width(), combined.height());
    write_jpeg(&combined, "output_combined.jpg", 90)?;
    println!("Saved: output_combined.jpg");

    // Test with actual image file if available
    println!("\n--- Optional: Test with Real Image ---");
    println!("Attempting to load 'test.jpg' if it exists...");
    match read_jpeg("test.jpg") {
        Ok(real_image) => {
            println!(
                "Loaded real image: {}x{}",
                real_image.width(),
                real_image.height()
            );

            // Create a thumbnail
            let thumbnail = real_image.resize(320, 240, InterpolationMethod::Bilinear);
            write_jpeg(&thumbnail, "output_thumbnail.jpg", 85)?;
            println!("Created thumbnail: output_thumbnail.jpg");

            // Rotate the original 90 degrees
            let rotated = real_image.rotate(RotationAngle::Rotate90);
            write_jpeg(&rotated, "output_real_rotated.jpg", 90)?;
            println!("Created rotated image: output_real_rotated.jpg");

            // Custom rotate by 15 degrees
            let custom_rotated =
                real_image.rotate_custom(Rotation::Degrees(15.0), InterpolationMethod::Bilinear);
            write_jpeg(&custom_rotated, "output_real_custom_rotated.jpg", 90)?;
            println!("Created custom rotated image: output_real_custom_rotated.jpg");
        }
        Err(_) => {
            println!("No test.jpg found - skipping real image test");
            println!("(Create a test.jpg file to test with real images)");
        }
    }

    println!("\n=== Demo Complete ===");
    println!("Check the output files in the current directory.");

    Ok(())
}
