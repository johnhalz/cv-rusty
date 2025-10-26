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
| `window` | GUI window display support | - |

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

## Drawing Shapes

### Import

```rust
use cv_rusty::{draw_rectangle, draw_circle, Color, Stroke};
```

### Color Creation

```rust
// RGB colors
let red = Color::rgb(255, 0, 0);
let green = Color::rgb(0, 255, 0);
let blue = Color::rgb(0, 0, 255);

// Grayscale colors
let white = Color::gray(255);
let black = Color::gray(0);
let gray = Color::gray(128);

// Named colors
let black = Color::black();
let white = Color::white();

// Hex colors (6-digit or 3-digit format)
let orange = Color::from_hex("#FF5733")?;
let cyan = Color::from_hex("#0FF")?;
let gold = Color::from_hex("FFD700")?;  // # prefix optional

// Parse from string
let pink: Color = "#FF1493".parse()?;

// Colors with opacity (0.0 = transparent, 1.0 = opaque)
let semi_red = Color::rgb_with_opacity(255, 0, 0, 0.5);     // 50% transparent
let semi_gray = Color::gray_with_opacity(128, 0.7);         // 70% opaque

// Modify opacity of existing colors
let opaque_blue = Color::rgb(0, 0, 255);
let transparent_blue = opaque_blue.with_opacity(0.3);       // 30% opaque

// Get opacity value
let opacity = semi_red.opacity();  // Returns 0.5
```

### Draw Rectangle

```rust
use cv_rusty::{Matrix3, draw_rectangle, Color, Stroke};

let mut image = Matrix3::zeros(640, 480);

// Basic filled rectangle with border
draw_rectangle(
    &mut image,
    320.0, 240.0,    // Center position (x, y)
    100.0, 80.0,     // Width, height
    0.0,             // Rotation in degrees (clockwise)
    Some(Stroke::new(2, Color::rgb(0, 0, 0))),  // 2px black stroke
    Some(Color::rgb(255, 0, 0))                 // Red fill
);

// Rotated rectangle
draw_rectangle(
    &mut image,
    200.0, 150.0,
    80.0, 120.0,
    45.0,  // 45 degrees clockwise
    Some(Stroke::new(3, Color::white())),
    Some(Color::rgb(0, 255, 0))
);

// Outline only (no fill)
draw_rectangle(
    &mut image,
    400.0, 300.0,
    150.0, 100.0,
    30.0,
    Some(Stroke::new(4, Color::rgb(0, 0, 255))),
    None  // No fill
);

// Fill only (no stroke)
draw_rectangle(
    &mut image,
    500.0, 200.0,
    60.0, 60.0,
    0.0,
    None,  // No stroke
    Some(Color::rgb(255, 255, 0))
);
```

### Draw Circle

```rust
use cv_rusty::{Matrix3, draw_circle, Color, Stroke};

let mut image = Matrix3::zeros(640, 480);

// Basic filled circle with border
draw_circle(
    &mut image,
    320.0, 240.0,    // Center position (x, y)
    50.0,            // Radius
    Some(Stroke::new(3, Color::white())),  // 3px white stroke
    Some(Color::rgb(0, 0, 255))            // Blue fill
);

// Outline only
draw_circle(
    &mut image,
    200.0, 200.0,
    60.0,
    Some(Stroke::new(5, Color::rgb(255, 0, 0))),  // 5px red stroke
    None  // No fill
);

// Fill only
draw_circle(
    &mut image,
    450.0, 350.0,
    40.0,
    None,  // No stroke
    Some(Color::rgb(0, 255, 0))
);
```

### Works with Grayscale Images

```rust
use cv_rusty::{Matrix1, draw_rectangle, draw_circle, Color, Stroke};

let mut gray_image = Matrix1::zeros(640, 480);

// Same functions work with grayscale images
draw_rectangle(
    &mut gray_image,
    320.0, 240.0,
    100.0, 60.0,
    0.0,
    Some(Stroke::new(2, Color::gray(255))),  // White stroke
    Some(Color::gray(100))                   // Dark gray fill
);

draw_circle(
    &mut gray_image,
    320.0, 240.0,
    50.0,
    Some(Stroke::new(3, Color::gray(255))),
    Some(Color::gray(100))
);
```

### Using Hex Colors

```rust
// Draw with hex colors
draw_rectangle(
    &mut image,
    320.0, 120.0,
    100.0, 60.0,
    15.0,
    Some(Stroke::new(2, Color::from_hex("#2C3E50")?)),  // Dark blue-gray
    Some(Color::from_hex("#3498DB")?)                   // Light blue
);

// 3-digit hex format (expands F -> FF)
draw_circle(
    &mut image,
    200.0, 200.0,
    40.0,
    Some(Stroke::new(2, Color::from_hex("#000")?)),  // Black
    Some(Color::from_hex("#F0F")?)                    // Magenta
);
```

### Drawing Patterns

```rust
// Draw grid of circles
for i in 0..5 {
    let x = 100.0 + (i as f32 * 120.0);
    draw_circle(
        &mut image,
        x, 240.0,
        40.0,
        Some(Stroke::new(3, Color::black())),
        Some(Color::rgb(100, 200, 100))
    );
}

// Draw rotating rectangles
for angle in (0..360).step_by(45) {
    draw_rectangle(
        &mut image,
        320.0, 240.0,
        100.0, 50.0,
        angle as f32,
        Some(Stroke::new(2, Color::rgb(255, 0, 0))),
        None
    );
}
```

### Annotation Example

```rust
// Draw bounding box annotation
fn draw_bbox(image: &mut Matrix3, x: f32, y: f32, 
             w: f32, h: f32, confidence: f32) {
    let color = if confidence > 0.8 {
        Color::rgb(0, 255, 0)  // Green
    } else if confidence > 0.5 {
        Color::rgb(255, 255, 0)  // Yellow
    } else {
        Color::rgb(255, 0, 0)  // Red
    };
    
    draw_rectangle(
        image,
        x + w / 2.0,
        y + h / 2.0,
        w, h,
        0.0,
        Some(Stroke::new(3, color)),
        None
    );
}
```

### Opacity and Transparency

```rust
// Draw overlapping semi-transparent shapes
draw_rectangle(
    &mut image,
    200.0, 200.0,
    150.0, 100.0,
    0.0,
    None,
    Some(Color::rgb(0, 0, 255))  // Opaque blue
);

draw_rectangle(
    &mut image,
    250.0, 220.0,
    150.0, 100.0,
    0.0,
    None,
    Some(Color::rgb_with_opacity(255, 0, 0, 0.5))  // 50% transparent red
);
// Overlap shows purple blend

// Venn diagram with color blending
let colors = [
    Color::rgb_with_opacity(255, 0, 0, 0.6),    // Red
    Color::rgb_with_opacity(0, 255, 0, 0.6),    // Green
    Color::rgb_with_opacity(0, 0, 255, 0.6),    // Blue
];

for (i, color) in colors.iter().enumerate() {
    draw_circle(
        &mut image,
        200.0 + (i as f32 * 60.0),
        240.0,
        50.0,
        None,
        Some(*color)
    );
}

// Watermark effect with very low opacity
draw_rectangle(
    &mut image,
    400.0, 500.0,
    300.0, 80.0,
    15.0,
    None,
    Some(Color::rgb_with_opacity(128, 128, 128, 0.15))  // 15% opacity
);
```

### Performance

| Operation | Time (640×480) | Complexity |
|-----------|---------------|------------|
| Rectangle (aligned) | ~30 µs | O(bbox_area) |
| Rectangle (rotated) | ~45 µs | O(bbox_area) |
| Circle (r=50) | ~50 µs | O(bbox_area) |
| 100 shapes | ~4 ms | - |

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

## Window Display API (requires `window` feature)

### Enable Feature

```toml
[dependencies]
cv-rusty = { version = "0.3.0", features = ["window"] }
```

### Import

```rust
use cv_rusty::{show_image, show_and_wait, wait_key, Displayable, WindowError};
```

### Display Images (Works with Matrix1 and Matrix3)

```rust
use cv_rusty::{Matrix1, Matrix3, show_image};

// Display a color image
let color_image = Matrix3::zeros(640, 480);
show_image("Color Window", &color_image)?;

// Display a grayscale image
let gray_image = Matrix1::zeros(640, 480);
show_image("Grayscale Window", &gray_image)?;
```

### Display and Wait Functions

```rust
// Display image and wait for user to close (works with any image type)
show_and_wait("My Window", &image)?;

// Wait for specified milliseconds
wait_key(1000); // Wait 1 second
wait_key(0);    // Wait indefinitely
```

### Window Error Types

```rust
pub enum WindowError {
    WindowCreation(String),  // Failed to create/update window
    InvalidDimensions,       // Image has zero width or height
}
```

### Complete Window Example

```rust
use cv_rusty::{Matrix3, show_image, read_jpeg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let image = read_jpeg("input.jpg")?;
    
    // Display image
    show_image("My Image", &image)?;
    
    Ok(())
}
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| ESC | Close window |
| Window X button | Close window |

### Display Multiple Images Sequentially

```rust
show_image("Image 1", &image1)?;  // Shows, waits until closed
show_image("Image 2", &image2)?;  // Then shows next
show_image("Image 3", &image3)?;  // And so on...
```

### Display with Error Handling

```rust
match show_image("Window", &image) {
    Ok(_) => println!("Displayed successfully"),
    Err(WindowError::InvalidDimensions) => {
        eprintln!("Invalid image dimensions");
    }
    Err(WindowError::WindowCreation(msg)) => {
        eprintln!("Window error: {}", msg);
    }
}
```

### Display Processed Image

```rust
// Original
let image = read_jpeg("input.jpg")?;
show_image("Original", &image)?;

// Apply processing
let kernel = Kernel::gaussian(5, 1.0);
let blurred = image.convolve(&kernel, BorderMode::Replicate);
show_image("Blurred", &blurred)?;
```

### Create and Display Test Pattern

```rust
let mut image = Matrix3::zeros(400, 300);

// Draw red square
for y in 100..200 {
    for x in 150..250 {
        image.set_pixel(x, y, 255, 0, 0);
    }
}

show_image("Test Pattern", &image)?;
```

### Display Both Color and Grayscale

```rust
// Single function works for both types
let color_image = Matrix3::zeros(640, 480);
let gray_image = Matrix1::zeros(640, 480);

show_image("Color", &color_image)?;
show_image("Grayscale", &gray_image)?;
```

### Run Window Examples

```bash
# Simple example
cargo run --example simple_show_image --features window

# Comprehensive example  
cargo run --example window_display_example --features window
```

### Window Display Notes

- Windows are displayed sequentially (blocking)
- Each window runs at maximum 60 FPS
- Requires GUI support (not for headless environments)
- Single `show_image()` function works with both color and grayscale images
- Uses Rust's trait system (`Displayable` trait) for type safety
- Image data format: RGB for Matrix3, grayscale for Matrix1

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

# Build with window feature
cargo build --features window

# Build for no_std
cargo build --no-default-features

# Run tests
cargo test

# Run examples
cargo run --example read_jpeg_example image.jpg
cargo run --example read_png_example image.png
cargo run --example no_std_example
cargo run --example drawing_example
cargo run --example drawing_hex_colors
cargo run --example drawing_grayscale_example
cargo run --example drawing_quick_test
cargo run --example simple_show_image --features window
cargo run --example window_display_example --features window
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
