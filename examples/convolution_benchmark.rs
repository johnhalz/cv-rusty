//! Benchmark comparing sequential vs parallel convolution performance.
//!
//! Run with:
//! ```
//! cargo run --release --example convolution_benchmark
//! cargo run --release --example convolution_benchmark --no-default-features --features std
//! ```

use cv_rusty::{read_jpeg, BorderMode, Kernel, Matrix3};
use std::time::Instant;

fn main() {
    println!("=== Convolution Performance Benchmark ===\n");

    // Try to load a test image, or create a synthetic one
    let image = match read_jpeg("examples/input.jpg") {
        Ok(img) => {
            println!("Loaded test image: {}x{}", img.width(), img.height());
            img
        }
        Err(_) => {
            println!("No test image found, creating synthetic 1920x1080 image");
            Matrix3::new(1920, 1080, vec![128u8; 1920 * 1080 * 3])
        }
    };

    let width = image.width();
    let height = image.height();
    let pixels = width * height;

    println!("Image size: {}x{} ({} pixels)\n", width, height, pixels);

    #[cfg(feature = "parallel")]
    {
        println!("✓ Parallel processing ENABLED (using rayon)");
    }
    #[cfg(not(feature = "parallel"))]
    {
        println!("✗ Parallel processing DISABLED (sequential execution)");
    }

    println!("\n--- Small Kernel (3x3) ---");
    benchmark_kernel(&image, &Kernel::sobel_x(), "Sobel X");
    benchmark_kernel(&image, &Kernel::sharpen(), "Sharpen");
    benchmark_kernel(&image, &Kernel::laplacian(), "Laplacian");

    println!("\n--- Medium Kernel (5x5) ---");
    benchmark_kernel(&image, &Kernel::gaussian(5, 1.0), "Gaussian 5x5");
    benchmark_kernel(&image, &Kernel::box_blur(5), "Box Blur 5x5");

    println!("\n--- Large Kernel (9x9) ---");
    benchmark_kernel(&image, &Kernel::gaussian(9, 2.0), "Gaussian 9x9");
    benchmark_kernel(&image, &Kernel::box_blur(9), "Box Blur 9x9");

    println!("\n--- Very Large Kernel (15x15) ---");
    benchmark_kernel(&image, &Kernel::gaussian(15, 3.0), "Gaussian 15x15");

    println!("\n--- Separable Convolution (more efficient for large kernels) ---");
    benchmark_separable(&image, 5, 1.0, "Gaussian 5x5");
    benchmark_separable(&image, 9, 2.0, "Gaussian 9x9");
    benchmark_separable(&image, 15, 3.0, "Gaussian 15x15");
    benchmark_separable(&image, 21, 4.0, "Gaussian 21x21");

    println!("\n--- Border Mode Comparison (5x5 Gaussian) ---");
    let kernel = Kernel::gaussian(5, 1.0);
    benchmark_border_mode(&image, &kernel, BorderMode::Zero, "Zero");
    benchmark_border_mode(&image, &kernel, BorderMode::Replicate, "Replicate");
    benchmark_border_mode(&image, &kernel, BorderMode::Reflect, "Reflect");
    benchmark_border_mode(&image, &kernel, BorderMode::Wrap, "Wrap");

    println!("\n--- Performance Summary ---");
    let kernel = Kernel::gaussian(9, 2.0);
    let iterations = 5;
    let mut times = Vec::new();

    for i in 0..iterations {
        let start = Instant::now();
        let _ = image.convolve(&kernel, BorderMode::Replicate);
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;
        times.push(elapsed);
        println!("Run {}: {:.2}ms", i + 1, elapsed);
    }

    let avg_time = times.iter().sum::<f64>() / times.len() as f64;
    let min_time = times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_time = times.iter().fold(0.0f64, |a, &b| a.max(b));

    println!("\nAverage: {:.2}ms", avg_time);
    println!("Min: {:.2}ms", min_time);
    println!("Max: {:.2}ms", max_time);

    let mpixels_per_sec = (pixels as f64 / 1_000_000.0) / (min_time / 1000.0);
    println!("\nThroughput: {:.2} Mpixels/sec", mpixels_per_sec);
}

fn benchmark_kernel(image: &Matrix3, kernel: &Kernel, name: &str) {
    let start = Instant::now();
    let _ = image.convolve(kernel, BorderMode::Replicate);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    let ops = image.width() * image.height() * kernel.width() * kernel.height() * 3;
    let mops = ops as f64 / 1_000_000.0;
    let mops_per_sec = mops / (elapsed / 1000.0);

    println!(
        "{:<20} {}x{}: {:>8.2}ms ({:.0} MOps/sec)",
        name,
        kernel.width(),
        kernel.height(),
        elapsed,
        mops_per_sec
    );
}

fn benchmark_separable(image: &Matrix3, size: usize, sigma: f32, name: &str) {
    let kernel_1d = create_gaussian_1d(size, sigma);

    let start = Instant::now();
    let _ = image.convolve_separable(&kernel_1d, &kernel_1d, BorderMode::Replicate);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    // Separable: 2 * (width * height * kernel_size) operations per channel
    let ops = 2 * image.width() * image.height() * size * 3;
    let mops = ops as f64 / 1_000_000.0;
    let mops_per_sec = mops / (elapsed / 1000.0);

    // Compare with full 2D convolution
    let ops_2d = image.width() * image.height() * size * size * 3;
    let speedup = ops_2d as f64 / ops as f64;

    println!(
        "{:<20} {}x1 + 1x{}: {:>8.2}ms ({:.0} MOps/sec, {:.1}x speedup vs 2D)",
        name, size, size, elapsed, mops_per_sec, speedup
    );
}

fn benchmark_border_mode(image: &Matrix3, kernel: &Kernel, mode: BorderMode, name: &str) {
    let start = Instant::now();
    let _ = image.convolve(kernel, mode);
    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    println!("{:<20} {:>8.2}ms", name, elapsed);
}

fn create_gaussian_1d(size: usize, sigma: f32) -> Vec<f32> {
    assert!(size % 2 == 1, "Size must be odd");
    let half = (size / 2) as i32;
    let mut kernel = Vec::with_capacity(size);
    let mut sum = 0.0;

    for i in -half..=half {
        let coefficient = 1.0 / (core::f32::consts::TAU.sqrt() * sigma);
        let exponent = -(i as f32 * i as f32) / (2.0 * sigma * sigma);
        let value = coefficient * exponent.exp();
        kernel.push(value);
        sum += value;
    }

    // Normalize
    for value in &mut kernel {
        *value /= sum;
    }

    kernel
}
