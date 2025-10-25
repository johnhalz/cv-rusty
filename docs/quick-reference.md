# CV Rusty Quick Reference

## Installation

### With Standard Library (default)
```toml
[dependencies]
cv-rusty = "0.1.0"
```

### For Embedded/no_std
```toml
[dependencies]
cv-rusty = { version = "0.1.0", default-features = false }
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `std` | Standard library support + file I/O | ✓ |
| `alloc` | Heap allocation (required for core) | - |

## Basic Operations

### Creating Images

```rust
use cv_rusty::Matrix3;

// Create zero-filled image
let image = Matrix3::zeros(640, 480);

// Create from raw RGB data
let data = vec![0u8; 640 * 480 * 3];
let image = Matrix3::new(640, 480, data);
```

### Reading Images (requires `std`)

```rust
use cv_rusty::io::{read_jpeg, read_png};

// Read JPEG
let image = read_jpeg("photo.jpg")?;

// Read PNG
let image = read_png("photo.png")?;
```

### Accessing Image Properties

```rust
let width = image.width();
let height = image.height();
let (w, h) = image.dimensions();
let raw_data = image.data();
```

### Pixel Operations

```rust
// Get pixel
if let Some((r, g, b)) = image.get_pixel(10, 20) {
    println!("RGB: ({}, {}, {})", r, g, b);
}

// Set pixel
image.set_pixel(10, 20, 255, 0, 0); // Red pixel
```

### Direct Data Access

```rust
// Read-only access
let data = image.data();
for chunk in data.chunks_exact(3) {
    let (r, g, b) = (chunk[0], chunk[1], chunk[2]);
    // Process pixel...
}

// Mutable access
let data = image.data_mut();
for pixel in data.iter_mut() {
    *pixel = (*pixel as u16 * 2).min(255) as u8;
}
```

## Image Transformations

### Resize

```rust
use cv_rusty::{Matrix3, InterpolationMethod};

// Resize with bilinear interpolation (recommended)
let resized = image.resize(320, 240, InterpolationMethod::Bilinear);

// Resize with nearest neighbor (faster, lower quality)
let resized = image.resize(320, 240, InterpolationMethod::NearestNeighbor);

// Upscale
let enlarged = image.resize(1280, 720, InterpolationMethod::Bilinear);

// Downscale
let thumbnail = image.resize(80, 60, InterpolationMethod::Bilinear);
```

### Crop

```rust
// Crop region: (x, y, width, height)
let cropped = image.crop(100, 100, 200, 200).unwrap();

// Center crop
let (w, h) = image.dimensions();
let crop_w = 400;
let crop_h = 300;
let x = (w - crop_w) / 2;
let y = (h - crop_h) / 2;
let center_crop = image.crop(x, y, crop_w, crop_h).unwrap();

// Handle errors
match image.crop(50, 50, 200, 200) {
    Some(cropped) => println!("Crop successful"),
    None => println!("Invalid crop region"),
}
```

### Rotate

```rust
use cv_rusty::{RotationAngle, Rotation, InterpolationMethod};

// Rotate 90 degrees clockwise (fast, lossless)
let rotated = image.rotate(RotationAngle::Rotate90);

// Rotate 180 degrees (fast, lossless)
let flipped = image.rotate(RotationAngle::Rotate180);

// Rotate 270 degrees clockwise (fast, lossless)
let rotated_ccw = image.rotate(RotationAngle::Rotate270);

// Rotate by arbitrary angle with degrees
let rotated_45 = image.rotate_custom(
    Rotation::Degrees(45.0),
    InterpolationMethod::Bilinear
);

// Rotate by arbitrary angle with radians
let rotated_pi4 = image.rotate_custom(
    Rotation::Radians(std::f32::consts::PI / 4.0),
    InterpolationMethod::Bilinear
);

// Counter-clockwise rotation with negative angle
let rotated_ccw = image.rotate_custom(
    Rotation::Degrees(-30.0),
    InterpolationMethod::Bilinear
);

// Fast nearest neighbor for arbitrary angles
let rotated_fast = image.rotate_custom(
    Rotation::Degrees(15.0),
    InterpolationMethod::NearestNeighbor
);
```

### Chaining Transformations

```rust
// Create a thumbnail: crop center, resize, and rotate
let thumbnail = image
    .crop(100, 100, 400, 300)
    .unwrap()
    .resize(200, 150, InterpolationMethod::Bilinear)
    .rotate(RotationAngle::Rotate90);

// Process and save
let processed = image
    .resize(640, 480, InterpolationMethod::Bilinear)
    .crop(50, 50, 500, 350)
    .unwrap();
write_jpeg(&processed, "output.jpg", 90)?;
```

## Common Patterns

### Brightness Adjustment

```rust
fn adjust_brightness(image: &mut Matrix3, factor: f32) {
    for pixel in image.data_mut().iter_mut() {
        *pixel = (*pixel as f32 * factor).min(255.0) as u8;
    }
}
```

### Grayscale Conversion

```rust
fn to_grayscale(image: &Matrix3) -> Matrix3 {
    let (w, h) = image.dimensions();
    let mut gray = Matrix3::zeros(w, h);
    
    for y in 0..h {
        for x in 0..w {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let gray_val = (0.299 * r as f32 + 
                               0.587 * g as f32 + 
                               0.114 * b as f32) as u8;
                gray.set_pixel(x, y, gray_val, gray_val, gray_val);
            }
        }
    }
    gray
}
```

### Region of Interest (ROI)

```rust
fn extract_roi(src: &Matrix3, x: usize, y: usize, 
               w: usize, h: usize) -> Matrix3 {
    let mut roi = Matrix3::zeros(w, h);
    for dy in 0..h {
        for dx in 0..w {
            if let Some((r, g, b)) = src.get_pixel(x + dx, y + dy) {
                roi.set_pixel(dx, dy, r, g, b);
            }
        }
    }
    roi
}
```

### Color Channel Operations

```rust
// Extract red channel
fn extract_red_channel(image: &Matrix3) -> Matrix3 {
    let (w, h) = image.dimensions();
    let mut red = Matrix3::zeros(w, h);
    
    for y in 0..h {
        for x in 0..w {
            if let Some((r, _, _)) = image.get_pixel(x, y) {
                red.set_pixel(x, y, r, 0, 0);
            }
        }
    }
    red
}
```

## Error Handling

### With Match

```rust
use cv_rusty::io::{read_jpeg, read_png, ImageError};

match read_png("photo.png") {
    Ok(image) => println!("Loaded {}x{}", image.width(), image.height()),
    Err(ImageError::Io(e)) => eprintln!("I/O error: {}", e),
    Err(ImageError::JpegDecode(e)) => eprintln!("JPEG error: {}", e),
    Err(ImageError::PngDecode(e)) => eprintln!("PNG error: {}", e),
    Err(ImageError::UnsupportedFormat(e)) => eprintln!("Format error: {}", e),
}
```

### With ? Operator

```rust
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let jpeg_image = read_jpeg("photo.jpg")?;
    let png_image = read_png("photo.png")?;
    // Process images...
    Ok(())
}
```

## Embedded/no_std Usage

### Setup

```rust
#![no_std]
extern crate alloc;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init() {
    const HEAP_SIZE: usize = 64 * 1024;
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
```

### Create Small Images

```rust
use cv_rusty::Matrix3;

// 80x60 image = ~14 KB
let image = Matrix3::zeros(80, 60);
```

### Process in Chunks

```rust
const CHUNK_HEIGHT: usize = 10;

for start_y in (0..height).step_by(CHUNK_HEIGHT) {
    let h = CHUNK_HEIGHT.min(height - start_y);
    let chunk = Matrix3::zeros(width, h);
    // Process chunk...
}
```

## Memory Requirements

| Resolution | Memory |
|------------|--------|
| 80×60 | ~14 KB |
| 160×120 | ~57 KB |
| 320×240 | ~230 KB |
| 640×480 | ~921 KB |

Formula: `width × height × 3 bytes + 24 bytes overhead`

## Performance Tips

1. **Use direct data access** for bulk operations
2. **Process in row-major order** for cache efficiency
3. **Reuse buffers** instead of creating new images
4. **Modify in-place** when possible
5. **Batch pixel operations** to reduce overhead

## Build Commands

```bash
# Build with std (default)
cargo build

# Build for no_std
cargo build --no-default-features

# Run tests
cargo test

# Run examples
cargo run --example read_jpeg_example image.jpg
cargo run --example read_png_example image.png
cargo run --example no_std_example
```

## Documentation

```bash
# Generate and open docs
cargo doc --open
```

## Support

- GitHub: [Repository URL]
- Documentation: Run `cargo doc --open`
- Examples: See `examples/` directory
- Architecture: See `ARCHITECTURE.md`
- Embedded Guide: See `EMBEDDED.md`
