# Convolution Operations in CV Rusty

This guide covers the efficient convolution operations available in cv-rusty, including parallel processing support and performance optimization techniques.

## Overview

Convolution is a fundamental operation in image processing used for:

- **Blurring**: Smoothing images and reducing noise
- **Edge Detection**: Finding boundaries and features
- **Sharpening**: Enhancing image details
- **Custom Effects**: Embossing, motion blur, etc.

CV Rusty provides a highly optimized convolution implementation with:

- ✅ Parallel processing support (automatic multi-threading)
- ✅ Multiple border handling modes
- ✅ Separable convolution for performance
- ✅ Built-in common kernels
- ✅ Custom kernel support
- ✅ `no_std` compatible

## Quick Start

```rust
use cv_rusty::{read_jpeg, write_jpeg, Kernel, BorderMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let image = read_jpeg("input.jpg")?;

    // Apply Gaussian blur
    let kernel = Kernel::gaussian(5, 1.0);
    let blurred = image.convolve(&kernel, BorderMode::Replicate);

    // Save result
    write_jpeg(&blurred, "output.jpg", 90)?;

    Ok(())
}
```

## Built-in Kernels

### Gaussian Blur

Smooth blur that preserves edges better than box blur.

```rust
// Create 5x5 Gaussian kernel with sigma=1.0
let kernel = Kernel::gaussian(5, 1.0);
let blurred = image.convolve(&kernel, BorderMode::Replicate);

// Larger kernel = stronger blur
let kernel = Kernel::gaussian(9, 2.0);
let very_blurred = image.convolve(&kernel, BorderMode::Replicate);
```

**Parameters:**

- `size`: Kernel size (must be odd: 3, 5, 7, 9, etc.)
- `sigma`: Standard deviation (controls blur strength)

**Guidelines:**

- `sigma ≈ size/6` for good coverage
- Larger sigma = stronger blur
- Use separable convolution for kernels > 5×5

### Box Blur

Uniform averaging filter (faster but less smooth than Gaussian).

```rust
let kernel = Kernel::box_blur(5);
let blurred = image.convolve(&kernel, BorderMode::Replicate);
```

### Sobel Edge Detection

Detects edges in horizontal or vertical directions.

```rust
// Detect horizontal edges
let sobel_x = Kernel::sobel_x();
let edges_x = image.convolve(&sobel_x, BorderMode::Replicate);

// Detect vertical edges
let sobel_y = Kernel::sobel_y();
let edges_y = image.convolve(&sobel_y, BorderMode::Replicate);

// Combine for gradient magnitude
let magnitude = combine_gradients(&edges_x, &edges_y);
```

**Combine Gradients Example:**
```rust
fn combine_gradients(gx: &Matrix3, gy: &Matrix3) -> Matrix3 {
    let width = gx.width();
    let height = gx.height();
    let mut result_data = Vec::with_capacity(width * height * 3);

    for y in 0..height {
        for x in 0..width {
            let (gx_r, gx_g, gx_b) = gx.get_pixel(x, y).unwrap();
            let (gy_r, gy_g, gy_b) = gy.get_pixel(x, y).unwrap();

            // Compute magnitude: sqrt(gx² + gy²)
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
```

### Laplacian

Omnidirectional edge detection.

```rust
let kernel = Kernel::laplacian();
let edges = image.convolve(&kernel, BorderMode::Replicate);
```

### Sharpen

Enhances image details and edges.

```rust
let kernel = Kernel::sharpen();
let sharpened = image.convolve(&kernel, BorderMode::Replicate);
```

## Custom Kernels

Create your own convolution kernels:

```rust
// Emboss effect
let emboss = Kernel::new(3, 3, vec![
    -2.0, -1.0,  0.0,
    -1.0,  1.0,  1.0,
     0.0,  1.0,  2.0,
]);
let result = image.convolve(&emboss, BorderMode::Replicate);

// Strong edge detection
let edge = Kernel::new(3, 3, vec![
    -1.0, -1.0, -1.0,
    -1.0,  8.0, -1.0,
    -1.0, -1.0, -1.0,
]);
let edges = image.convolve(&edge, BorderMode::Zero);

// Custom blur
let custom_blur = Kernel::new(3, 3, vec![
    1.0/16.0, 2.0/16.0, 1.0/16.0,
    2.0/16.0, 4.0/16.0, 2.0/16.0,
    1.0/16.0, 2.0/16.0, 1.0/16.0,
]);
let blurred = image.convolve(&custom_blur, BorderMode::Replicate);
```

**Requirements:**

- Width and height must be odd (3, 5, 7, 9, etc.)
- Data length must equal width × height

## Border Modes

Control how pixels outside the image boundary are handled:

### BorderMode::Replicate (Recommended)

Replicates the edge pixels. Best for most cases.

```rust
let result = image.convolve(&kernel, BorderMode::Replicate);
```

**Visual:**
```
Image:  | a b c d e |
Border: a a b c d e e e
```

### BorderMode::Zero

Pads with zeros (black). Can create dark edges.

```rust
let result = image.convolve(&kernel, BorderMode::Zero);
```

**Visual:**
```
Image:  | a b c d e |
Border: 0 0 a b c d e 0 0
```

### BorderMode::Reflect

Mirrors the image at the boundary. Good for seamless tiling.

```rust
let result = image.convolve(&kernel, BorderMode::Reflect);
```

**Visual:**
```
Image:  | a b c d e |
Border: c b a b c d e d c
```

### BorderMode::Wrap

Wraps around to the opposite edge. For periodic patterns.

```rust
let result = image.convolve(&kernel, BorderMode::Wrap);
```

**Visual:**
```
Image:  | a b c d e |
Border: d e a b c d e a b
```

## Separable Convolution

For separable kernels (like Gaussian), use `convolve_separable()` for massive performance gains.

### Why Separable?

A 2D convolution with an N×N kernel requires N² operations per pixel. A separable kernel can be decomposed into two 1D convolutions, requiring only 2N operations.

**Performance Gain:**

- 3×3: 1.5× faster
- 5×5: 2.5× faster
- 9×9: 4.5× faster
- 15×15: 7.5× faster
- 21×21: 10.5× faster

### Usage

```rust
// Create 1D Gaussian kernel
let kernel_1d = create_gaussian_1d(9, 2.0);

// Apply separable convolution (much faster!)
let blurred = image.convolve_separable(
    &kernel_1d,
    &kernel_1d,
    BorderMode::Replicate
);

// Helper function to create 1D Gaussian
fn create_gaussian_1d(size: usize, sigma: f32) -> Vec<f32> {
    assert!(size % 2 == 1);
    let half = (size / 2) as i32;
    let mut kernel = Vec::with_capacity(size);
    let mut sum = 0.0;

    for i in -half..=half {
        let coeff = 1.0 / (core::f32::consts::TAU.sqrt() * sigma);
        let exp = -(i as f32 * i as f32) / (2.0 * sigma * sigma);
        let value = coeff * exp.exp();
        kernel.push(value);
        sum += value;
    }

    // Normalize
    for value in &mut kernel {
        *value /= sum;
    }

    kernel
}
```

### When to Use Separable Convolution

✅ **Use separable when:**

- Gaussian blur (always separable)
- Box blur (always separable)
- Large kernel sizes (> 5×5)
- You care about performance

❌ **Don't use separable for:**

- Sobel (not separable in practice)
- Laplacian (not separable)
- Custom non-separable kernels
- Small kernels (3×3) where overhead may not be worth it

## Parallel Processing

By default, convolution uses all available CPU cores via Rayon.

### Enable/Disable

```toml
# Cargo.toml

# Enable parallel processing (default)
[dependencies]
cv-rusty = "0.2.0"

# Disable parallel processing
[dependencies]
cv-rusty = { version = "0.2.0", default-features = false, features = ["std"] }
```

### Performance Impact

On a 1920×1080 image with a 9×9 kernel:

- **Single-threaded:** ~200ms
- **Multi-threaded (8 cores):** ~30ms
- **Speedup:** ~6.7×

**Guidelines:**

- Parallel is best for large images (> 500×500)
- Parallel is best for large kernels (> 5×5)
- Small images may be faster single-threaded due to overhead
- Benchmark your specific use case

## Performance Tips

### 1. Always Use Release Mode

```bash
cargo build --release
cargo run --release --example convolution_benchmark
```

Debug mode can be **100× slower** than release mode!

### 2. Use Separable Convolution for Large Kernels

```rust
// ❌ Slow: O(n²) per pixel
let kernel = Kernel::gaussian(15, 3.0);
let result = image.convolve(&kernel, BorderMode::Replicate);

// ✅ Fast: O(2n) per pixel
let kernel_1d = create_gaussian_1d(15, 3.0);
let result = image.convolve_separable(&kernel_1d, &kernel_1d, BorderMode::Replicate);
```

### 3. Choose Appropriate Kernel Sizes

Larger kernels = stronger effect but slower:

| Kernel Size | Effect Strength | Speed (relative) |
|-------------|----------------|------------------|
| 3×3         | Minimal        | 1× (fastest)     |
| 5×5         | Light          | 3×               |
| 7×7         | Medium         | 5×               |
| 9×9         | Strong         | 9×               |
| 15×15       | Very Strong    | 25×              |

### 4. Reuse Kernels

```rust
// ❌ Recreates kernel each time
for image in images {
    let kernel = Kernel::gaussian(5, 1.0);
    let result = image.convolve(&kernel, BorderMode::Replicate);
}

// ✅ Reuse kernel
let kernel = Kernel::gaussian(5, 1.0);
for image in images {
    let result = image.convolve(&kernel, BorderMode::Replicate);
}
```

### 5. Process in Batches

For many images, use parallel iteration:

```rust
use rayon::prelude::*;

let results: Vec<Matrix3> = images
    .par_iter()
    .map(|img| img.convolve(&kernel, BorderMode::Replicate))
    .collect();
```

## Working with Grayscale Images

Convolution works the same for `Matrix1` (grayscale):

```rust
use cv_rusty::{Matrix1, Kernel, BorderMode};

// Convert to grayscale first
let gray = image.to_grayscale();

// Apply convolution
let kernel = Kernel::gaussian(5, 1.0);
let blurred = gray.convolve(&kernel, BorderMode::Replicate);

// Grayscale is ~3× faster than RGB (single channel)
```

## Common Patterns

### Blur then Edge Detection

```rust
// Reduce noise before edge detection
let kernel = Kernel::gaussian(3, 0.5);
let smoothed = image.convolve(&kernel, BorderMode::Replicate);

let sobel_x = Kernel::sobel_x();
let edges = smoothed.convolve(&sobel_x, BorderMode::Replicate);
```

### Unsharp Mask (Advanced Sharpening)

```rust
// 1. Create blurred version
let kernel = Kernel::gaussian(5, 1.0);
let blurred = image.convolve(&kernel, BorderMode::Replicate);

// 2. Subtract blurred from original (implement pixel-wise operations)
// 3. Add difference back to original with weight
```

### Multi-scale Processing

```rust
// Apply different kernel sizes
let blur_small = image.convolve(&Kernel::gaussian(3, 0.5), BorderMode::Replicate);
let blur_medium = image.convolve(&Kernel::gaussian(5, 1.0), BorderMode::Replicate);
let blur_large = image.convolve(&Kernel::gaussian(9, 2.0), BorderMode::Replicate);
```

## Benchmarking

Run the benchmark example to test performance on your hardware:

```bash
# With parallel processing
cargo run --release --example convolution_benchmark

# Without parallel processing
cargo run --release --example convolution_benchmark --no-default-features --features std
```

Expected output:
```
=== Convolution Performance Benchmark ===

Image size: 1920x1080 (2073600 pixels)
✓ Parallel processing ENABLED (using rayon)

--- Small Kernel (3x3) ---
Sobel X              3x3:     8.45ms (2204 MOps/sec)
Sharpen              3x3:     8.32ms (2238 MOps/sec)
Laplacian            3x3:     8.41ms (2214 MOps/sec)

--- Medium Kernel (5x5) ---
Gaussian 5x5         5x5:    18.23ms (2553 MOps/sec)
Box Blur 5x5         5x5:    17.89ms (2601 MOps/sec)

--- Large Kernel (9x9) ---
Gaussian 9x9         9x9:    52.67ms (2848 MOps/sec)
Box Blur 9x9         9x9:    51.34ms (2921 MOps/sec)

--- Separable Convolution ---
Gaussian 5x5         5x1 + 1x5:     9.12ms (2281 MOps/sec, 2.5x speedup vs 2D)
Gaussian 9x9         9x1 + 1x9:    16.45ms (2532 MOps/sec, 4.5x speedup vs 2D)
Gaussian 15x15       15x1 + 1x15:   27.89ms (2236 MOps/sec, 7.5x speedup vs 2D)

Throughput: 73.52 Mpixels/sec
```

## no_std Support

Convolution works in `no_std` environments:

```rust
#![no_std]

extern crate alloc;
use alloc::vec;
use cv_rusty::{Matrix3, Kernel, BorderMode};

fn process_camera_frame(frame: &Matrix3) -> Matrix3 {
    // Edge detection for embedded vision system
    let kernel = Kernel::sobel_x();
    frame.convolve(&kernel, BorderMode::Replicate)
}
```

**Note:** Parallel processing requires `std` and is automatically disabled in `no_std` builds.

## Examples

See the examples directory for complete working examples:

- `convolution_demo.rs` - Demonstrates all built-in kernels
- `convolution_benchmark.rs` - Performance testing
- Examples in `examples/README.md`

## API Reference

### Kernel Methods

```rust
impl Kernel {
    pub fn new(width: usize, height: usize, data: Vec<f32>) -> Self;
    pub fn box_blur(size: usize) -> Self;
    pub fn gaussian(size: usize, sigma: f32) -> Self;
    pub fn sobel_x() -> Self;
    pub fn sobel_y() -> Self;
    pub fn laplacian() -> Self;
    pub fn sharpen() -> Self;
    pub fn width(&self) -> usize;
    pub fn height(&self) -> usize;
    pub fn data(&self) -> &[f32];
}
```

### Matrix Methods

```rust
impl Matrix1 {
    pub fn convolve(&self, kernel: &Kernel, border_mode: BorderMode) -> Self;
    pub fn convolve_separable(&self, kernel_x: &[f32], kernel_y: &[f32], border_mode: BorderMode) -> Self;
}

impl Matrix3 {
    pub fn convolve(&self, kernel: &Kernel, border_mode: BorderMode) -> Self;
    pub fn convolve_separable(&self, kernel_x: &[f32], kernel_y: &[f32], border_mode: BorderMode) -> Self;
}
```

### BorderMode Enum

```rust
pub enum BorderMode {
    Zero,
    Replicate,
    Reflect,
    Wrap,
}
```

## Further Reading

- [Convolution on Wikipedia](https://en.wikipedia.org/wiki/Kernel_(image_processing))
- [Gaussian Blur](https://en.wikipedia.org/wiki/Gaussian_blur)
- [Sobel Operator](https://en.wikipedia.org/wiki/Sobel_operator)
- [Image Filtering Tutorial](https://setosa.io/ev/image-kernels/)
