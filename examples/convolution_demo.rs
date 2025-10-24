//! Convolution demo showing various image filters.
//!
//! This example demonstrates:
//! - Gaussian blur
//! - Edge detection (Sobel, Laplacian)
//! - Sharpening
//! - Box blur
//! - Separable convolution for efficiency

use cv_rusty::{read_jpeg, write_jpeg, BorderMode, Kernel, Matrix3};
use std::time::Instant;

fn main() {
    // Load an example image
    println!("Loading image...");
    let image = read_jpeg("examples/input.jpg").expect("Failed to load image");
    println!("Loaded {}x{} image", image.width(), image.height());

    // 1. Gaussian Blur
    println!("\n=== Gaussian Blur ===");
    let start = Instant::now();
    let kernel = Kernel::gaussian(5, 1.0);
    let blurred = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&blurred, "examples/output_gaussian.jpg", 95)
        .expect("Failed to write gaussian blur");

    // 2. Gaussian Blur using separable convolution (more efficient)
    println!("\n=== Gaussian Blur (Separable) ===");
    let start = Instant::now();
    let kernel_1d = create_gaussian_1d(5, 1.0);
    let blurred_sep = image.convolve_separable(&kernel_1d, &kernel_1d, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&blurred_sep, "examples/output_gaussian_sep.jpg", 95)
        .expect("Failed to write separable gaussian blur");

    // 3. Box Blur
    println!("\n=== Box Blur ===");
    let start = Instant::now();
    let kernel = Kernel::box_blur(5);
    let box_blurred = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&box_blurred, "examples/output_box.jpg", 95).expect("Failed to write box blur");

    // 4. Sobel X (horizontal edges)
    println!("\n=== Sobel X (Horizontal Edges) ===");
    let start = Instant::now();
    let kernel = Kernel::sobel_x();
    let sobel_x = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&sobel_x, "examples/output_sobel_x.jpg", 95).expect("Failed to write sobel x");

    // 5. Sobel Y (vertical edges)
    println!("\n=== Sobel Y (Vertical Edges) ===");
    let start = Instant::now();
    let kernel = Kernel::sobel_y();
    let sobel_y = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&sobel_y, "examples/output_sobel_y.jpg", 95).expect("Failed to write sobel y");

    // 6. Combined Sobel (edge magnitude)
    println!("\n=== Sobel Combined (Edge Magnitude) ===");
    let start = Instant::now();
    let sobel_combined = combine_sobel(&image);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&sobel_combined, "examples/output_sobel.jpg", 95)
        .expect("Failed to write combined sobel");

    // 7. Laplacian (edge detection)
    println!("\n=== Laplacian (Edge Detection) ===");
    let start = Instant::now();
    let kernel = Kernel::laplacian();
    let laplacian = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&laplacian, "examples/output_laplacian.jpg", 95).expect("Failed to write laplacian");

    // 8. Sharpen
    println!("\n=== Sharpen ===");
    let start = Instant::now();
    let kernel = Kernel::sharpen();
    let sharpened = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&sharpened, "examples/output_sharpen.jpg", 95).expect("Failed to write sharpened");

    // 9. Custom kernel - emboss
    println!("\n=== Emboss ===");
    let start = Instant::now();
    let kernel = Kernel::new(3, 3, vec![-2.0, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0]);
    let embossed = image.convolve(&kernel, BorderMode::Replicate);
    println!("Time: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&embossed, "examples/output_emboss.jpg", 95).expect("Failed to write emboss");

    // 10. Border mode comparison
    println!("\n=== Border Mode Comparison ===");
    let kernel = Kernel::gaussian(9, 2.0); // Large kernel to see border effects

    let start = Instant::now();
    let border_zero = image.convolve(&kernel, BorderMode::Zero);
    println!("Zero: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&border_zero, "examples/output_border_zero.jpg", 95)
        .expect("Failed to write border zero");

    let start = Instant::now();
    let border_replicate = image.convolve(&kernel, BorderMode::Replicate);
    println!("Replicate: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(
        &border_replicate,
        "examples/output_border_replicate.jpg",
        95,
    )
    .expect("Failed to write border replicate");

    let start = Instant::now();
    let border_reflect = image.convolve(&kernel, BorderMode::Reflect);
    println!("Reflect: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&border_reflect, "examples/output_border_reflect.jpg", 95)
        .expect("Failed to write border reflect");

    let start = Instant::now();
    let border_wrap = image.convolve(&kernel, BorderMode::Wrap);
    println!("Wrap: {:.2}ms", start.elapsed().as_secs_f64() * 1000.0);
    write_jpeg(&border_wrap, "examples/output_border_wrap.jpg", 95)
        .expect("Failed to write border wrap");

    println!("\nAll outputs saved to examples/ directory!");
}

/// Creates a 1D Gaussian kernel for separable convolution.
fn create_gaussian_1d(size: usize, sigma: f32) -> Vec<f32> {
    assert!(size % 2 == 1, "Size must be odd");
    let half = (size / 2) as i32;
    let mut kernel = Vec::with_capacity(size);
    let mut sum = 0.0;

    for i in -half..=half {
        let value = gaussian_1d(i as f32, sigma);
        kernel.push(value);
        sum += value;
    }

    // Normalize
    for value in &mut kernel {
        *value /= sum;
    }

    kernel
}

/// 1D Gaussian function.
fn gaussian_1d(x: f32, sigma: f32) -> f32 {
    let coefficient = 1.0 / (core::f32::consts::TAU.sqrt() * sigma);
    let exponent = -(x * x) / (2.0 * sigma * sigma);
    coefficient * exponent.exp()
}

/// Combines Sobel X and Y to compute edge magnitude.
fn combine_sobel(image: &Matrix3) -> Matrix3 {
    let sobel_x_kernel = Kernel::sobel_x();
    let sobel_y_kernel = Kernel::sobel_y();

    let gx = image.convolve(&sobel_x_kernel, BorderMode::Replicate);
    let gy = image.convolve(&sobel_y_kernel, BorderMode::Replicate);

    let width = image.width();
    let height = image.height();
    let mut result_data = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        for x in 0..width {
            let (gx_r, gx_g, gx_b) = gx.get_pixel(x, y).unwrap();
            let (gy_r, gy_g, gy_b) = gy.get_pixel(x, y).unwrap();

            // Compute magnitude for each channel
            let mag_r = ((gx_r as f32).powi(2) + (gy_r as f32).powi(2)).sqrt();
            let mag_g = ((gx_g as f32).powi(2) + (gy_g as f32).powi(2)).sqrt();
            let mag_b = ((gx_b as f32).powi(2) + (gy_b as f32).powi(2)).sqrt();

            result_data.push(mag_r.min(255.0) as u8);
            result_data.push(mag_g.min(255.0) as u8);
            result_data.push(mag_b.min(255.0) as u8);
        }
    }

    Matrix3::new(width, height, result_data)
}
