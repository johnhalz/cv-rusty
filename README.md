# CV Rusty

[![Run Tests](https://github.com/johnhalz/cv-rusty/actions/workflows/test.yaml/badge.svg)](https://github.com/johnhalz/cv-rusty/actions/workflows/test.yaml)
[![Build](https://github.com/johnhalz/cv-rusty/actions/workflows/build.yaml/badge.svg)](https://github.com/johnhalz/cv-rusty/actions/workflows/build.yaml)

A `no_std` computer vision library written in Rust, designed for live computations, embedded systems, and high-performance image processing.

## Documentation

Full documentation is available at: **[https://johnhalz.github.io/cv-rusty/](https://johnhalz.github.io/cv-rusty/)**

## Features

- **`no_std` Compatible**: Core library works without the standard library (only requires `alloc`)
- **Zero-copy Image Representation**: Efficient matrix structures for RGB (`Matrix3`) and grayscale (`Matrix1`) images
- **Convolution Operations**: Efficient 2D convolution with support for parallel processing when available
- **Built-in Kernels**: Gaussian blur, Sobel edge detection, Laplacian, sharpening, and more
- **Separable Convolution**: Optimized implementation for separable kernels (significantly faster for large kernels)
- **Parallel Processing**: Optional multi-threaded processing using Rayon (requires `parallel` feature)
- **Color Space Conversions**: Convert between RGB, HSV, and HSL color spaces; convert RGB to grayscale with multiple algorithms
- **Image Transformations**: Resize, crop, and rotate operations with multiple interpolation methods
- **Image I/O**: Built-in support for reading and writing JPEG and PNG images with automatic format conversion (requires `std` feature)</parameter>
- **Format Support**: Handles RGB24, Grayscale (L8), and CMYK32 JPEG formats; RGB, RGBA, Grayscale, and Grayscale+Alpha PNG formats
- **Safe API**: Bounds-checked pixel access with ergonomic error handling
- **Embedded Ready**: Perfect for resource-constrained environments and real-time systems

## Installation

### Standard Library with Parallel Processing (default)

For applications with `std` support, file I/O, and parallel processing:

```toml
[dependencies]
cv-rusty = "0.1.0"
```

### Standard Library without Parallel Processing

For applications with `std` support but without parallel processing:

```toml
[dependencies]
cv-rusty = { version = "0.1.0", default-features = false, features = ["std"] }
```

### `no_std` Environments

For embedded systems or `no_std` environments (requires `alloc`):

```toml
[dependencies]
cv-rusty = { version = "0.1.0", default-features = false }
```

## Feature Flags

- **`std`** (enabled by default): Enables standard library support, including file I/O operations
- **`parallel`** (enabled by default): Enables parallel processing for convolution operations using Rayon (requires `std`)
- **`alloc`**: Enables heap allocation support (required for core functionality)
- **`window`**: Enables GUI window support for displaying images with a unified `show_image()` API that works with both color and grayscale images

## Usage

### Reading and Writing Images (requires `std` feature)

```rust
use cv_rusty::io::{read_jpeg, read_png, write_jpeg, write_png};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a JPEG file into a Matrix3
    let image = read_jpeg("photo.jpg")?;
    println!("JPEG dimensions: {}x{}", image.width(), image.height());

    // Read a PNG file into a Matrix3
    let image = read_png("photo.png")?;
    println!("PNG dimensions: {}x{}", image.width(), image.height());

    // Access pixel data
    if let Some((r, g, b)) = image.get_pixel(100, 100) {
        println!("Pixel at (100, 100): RGB({}, {}, {})", r, g, b);
    }

    // Write as JPEG with quality setting (1-100)
    write_jpeg(&image, "output.jpg", 90)?;

    // Write as PNG (lossless)
    write_png(&image, "output.png")?;

    Ok(())
}

### Displaying Images in Windows (requires `window` feature)

```rust
use cv_rusty::{Matrix3, show_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create or load an image
    let mut image = Matrix3::zeros(400, 300);
    
    // Draw a red square
    for y in 100..200 {
        for x in 150..250 {
            image.set_pixel(x, y, 255, 0, 0);
        }
    }
    
    // Display the image
    // The show_image() function works with both Matrix3 (color) and Matrix1 (grayscale)
    // Window will close when user presses ESC or closes it
    show_image("My Window", &image)?;
    
    Ok(())
}
```

To enable this feature, add it to your `Cargo.toml`:
```toml
[dependencies]
cv-rusty = { version = "0.3.0", features = ["window"] }
```

```

### Color Space Conversions (`no_std` compatible)

```rust
use cv_rusty::{Matrix3, rgb_to_hsv, hsv_to_rgb, rgb_to_hsl, hsl_to_rgb};

// Convert RGB to HSV
let (h, s, v) = rgb_to_hsv(255, 0, 0); // Red
println!("Hue: {:.1}°, Saturation: {:.3}, Value: {:.3}", h, s, v);

// Convert HSV back to RGB
let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0); // Red
println!("RGB: ({}, {}, {})", r, g, b);

// Convert RGB to HSL
let (h, s, l) = rgb_to_hsl(255, 128, 0); // Orange
println!("Hue: {:.1}°, Saturation: {:.3}, Lightness: {:.3}", h, s, l);

// Convert HSL back to RGB
let (r, g, b) = hsl_to_rgb(30.0, 1.0, 0.5); // Orange
println!("RGB: ({}, {}, {})", r, g, b);
```

### Image Transformations (`no_std` compatible)

```rust
use cv_rusty::{Matrix3, InterpolationMethod, Rotation, RotationAngle};

// Load or create an image
let image = Matrix3::zeros(640, 480);

// Resize image with different interpolation methods
let resized_nn = image.resize(320, 240, InterpolationMethod::NearestNeighbor);
let resized_bilinear = image.resize(320, 240, InterpolationMethod::Bilinear);

// Crop a region (x, y, width, height)
let cropped = image.crop(100, 100, 200, 200).unwrap();

// Rotate image by 90-degree increments (fast, lossless)
let rotated_90 = image.rotate(RotationAngle::Rotate90);
let rotated_180 = image.rotate(RotationAngle::Rotate180);
let rotated_270 = image.rotate(RotationAngle::Rotate270);

// Rotate by arbitrary angle with interpolation
let rotated_45 = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
let rotated_pi4 = image.rotate_custom(Rotation::Radians(std::f32::consts::PI / 4.0), InterpolationMethod::Bilinear);

// Negative angles for counter-clockwise rotation
let rotated_ccw = image.rotate_custom(Rotation::Degrees(-30.0), InterpolationMethod::Bilinear);

// Chain operations
let thumbnail = image
    .crop(50, 50, 400, 300)
    .unwrap()
    .resize(200, 150, InterpolationMethod::Bilinear)
    .rotate(RotationAngle::Rotate90);
```

### Converting RGB to Grayscale (`no_std` compatible)</parameter>

```rust
use cv_rusty::{Matrix3, Matrix1, GrayscaleMethod};

// Create an RGB image
let mut rgb_image = Matrix3::zeros(640, 480);
rgb_image.set_pixel(10, 20, 255, 128, 64);

// Convert to grayscale using different methods:

// 1. Luminosity method (default, recommended)
// Formula: 0.299*R + 0.587*G + 0.114*B
let gray1 = rgb_image.to_grayscale();

// 2. Average method
// Formula: (R + G + B) / 3
let gray2 = rgb_image.to_grayscale_average();

// 3. Lightness method
// Formula: (max(R,G,B) + min(R,G,B)) / 2
let gray3 = rgb_image.to_grayscale_lightness();

// Or use the method parameter
let gray4 = rgb_image.to_grayscale_with_method(GrayscaleMethod::Luminosity);

// Access grayscale pixel values
if let Some(value) = gray1.get_pixel(10, 20) {
    println!("Grayscale value: {}", value);
}
```

### Convolution Operations

```rust
use cv_rusty::{Matrix3, Kernel, BorderMode};

// Load an image
let image = cv_rusty::read_jpeg("photo.jpg")?;

// Apply Gaussian blur
let kernel = Kernel::gaussian(5, 1.0);
let blurred = image.convolve(&kernel, BorderMode::Replicate);
cv_rusty::write_jpeg(&blurred, "blurred.jpg", 90)?;

// Apply Sobel edge detection
let sobel_x = Kernel::sobel_x();
let edges_x = image.convolve(&sobel_x, BorderMode::Replicate);

let sobel_y = Kernel::sobel_y();
let edges_y = image.convolve(&sobel_y, BorderMode::Replicate);

// Apply sharpening
let sharpen_kernel = Kernel::sharpen();
let sharpened = image.convolve(&sharpen_kernel, BorderMode::Replicate);

// Use separable convolution for better performance (for large kernels)
let kernel_1d = vec![0.25, 0.5, 0.25];
let blurred_fast = image.convolve_separable(&kernel_1d, &kernel_1d, BorderMode::Replicate);

// Create custom kernels
let custom = Kernel::new(3, 3, vec![
    -1.0, -1.0, -1.0,
    -1.0,  8.0, -1.0,
    -1.0, -1.0, -1.0,
]);
let result = image.convolve(&custom, BorderMode::Zero);
```

### Border Modes for Convolution

```rust
use cv_rusty::BorderMode;

// Zero: Pad with zeros outside boundary
let result = image.convolve(&kernel, BorderMode::Zero);

// Replicate: Repeat edge pixels (recommended for most cases)
let result = image.convolve(&kernel, BorderMode::Replicate);

// Reflect: Mirror across the edge
let result = image.convolve(&kernel, BorderMode::Reflect);

// Wrap: Wrap around to opposite edge
let result = image.convolve(&kernel, BorderMode::Wrap);
```

### Working with Matrix3 and Matrix1 (`no_std` compatible)

```rust
use cv_rusty::{Matrix3, Matrix1};

// Create a new 640x480 RGB image filled with zeros
let mut rgb_image = Matrix3::zeros(640, 480);

// Set a pixel value
rgb_image.set_pixel(10, 20, 255, 0, 0); // Red pixel at (10, 20)

// Get a pixel value
if let Some((r, g, b)) = rgb_image.get_pixel(10, 20) {
    println!("RGB: ({}, {}, {})", r, g, b);
}

// Access raw data
let raw_data = rgb_image.data();
println!("Total bytes: {}", raw_data.len());

// Create a grayscale image
let mut gray_image = Matrix1::zeros(640, 480);

// Set a grayscale pixel value
gray_image.set_pixel(10, 20, 128);

// Get a grayscale pixel value
if let Some(value) = gray_image.get_pixel(10, 20) {
    println!("Grayscale: {}", value);
}
```

### Error Handling

```rust
use cv_rusty::io::{read_jpeg, read_png, ImageError};

match read_png("photo.png") {
    Ok(image) => {
        println!("Successfully loaded {}x{} image", image.width(), image.height());
    }
    Err(ImageError::Io(e)) => {
        eprintln!("File I/O error: {}", e);
    }
    Err(ImageError::JpegDecode(e)) => {
        eprintln!("JPEG decoding error: {}", e);
    }
    Err(ImageError::PngDecode(e)) => {
        eprintln!("PNG decoding error: {}", e);
    }
    Err(ImageError::JpegEncode(e)) => {
        eprintln!("JPEG encoding error: {}", e);
    }
    Err(ImageError::PngEncode(e)) => {
        eprintln!("PNG encoding error: {}", e);
    }
    Err(ImageError::UnsupportedFormat(e)) => {
        eprintln!("Unsupported format: {}", e);
    }
}
```

### `no_std` Embedded Usage

```rust
#![no_std]

extern crate alloc;
use cv_rusty::Matrix3;

fn process_image() {
    // Create image data in memory
    let mut image = Matrix3::zeros(320, 240);

    // Process pixels (e.g., from a camera sensor)
    for y in 0..240 {
        for x in 0..320 {
            // Set pixel from sensor data
            image.set_pixel(x, y, r, g, b);
        }
    }

    // Send to display via SPI/I2C
    let raw_data = image.data();
    // ... send raw_data to hardware
}
```

## Examples

### With `std` feature (file I/O)

Read and analyze a JPEG file:

```bash
cargo run --example read_jpeg_example path/to/your/image.jpg
```

Read and analyze a PNG file:

```bash
cargo run --example read_png_example path/to/your/image.png
```

Write images in JPEG and PNG formats:

```bash
cargo run --example write_image_example
```

Convert images between formats:

```bash
cargo run --example image_conversion path/to/your/image.jpg
```

Demonstrate color space conversions:

```bash
cargo run --example color_conversion_example
```

Demonstrate image transformations (resize, crop, rotate):

```bash
cargo run --release --example transform_demo
```

### Convolution Examples

Apply various convolution filters to an image:

```bash
cargo run --release --example convolution_demo
```

Benchmark convolution performance (with/without parallel processing):

```bash
# With parallel processing
cargo run --release --example convolution_benchmark

# Without parallel processing
cargo run --release --example convolution_benchmark --no-default-features --features std
```

### `no_std` example

Demonstrate core functionality without file I/O:

```bash
cargo run --example no_std_example
```

Build for `no_std` environment:

```bash
cargo build --no-default-features
```

## API Documentation

### `Matrix3`

A three-channel matrix for representing RGB image data.

**Key Methods:**
- `new(width, height, data)` - Create from raw RGB data
- `zeros(width, height)` - Create a zero-initialized matrix
- `get_pixel(x, y)` - Get RGB values at a pixel location
- `set_pixel(x, y, r, g, b)` - Set RGB values at a pixel location
- `width()`, `height()`, `dimensions()` - Get matrix dimensions
- `data()`, `data_mut()` - Access raw pixel data
- `to_grayscale()` - Convert to grayscale using luminosity method
- `to_grayscale_average()` - Convert to grayscale using average method
- `to_grayscale_lightness()` - Convert to grayscale using lightness method
- `to_grayscale_with_method(method)` - Convert to grayscale with specified method
- `resize(width, height, method)` - Resize image with interpolation
- `crop(x, y, width, height)` - Crop image to specified region
- `rotate(angle)` - Rotate image by 90, 180, or 270 degrees (fast, lossless)
- `rotate_custom(angle, method)` - Rotate image by arbitrary angle with interpolation

### `Matrix1`

A single-channel matrix for representing grayscale image data.

**Key Methods:**
- `new(width, height, data)` - Create from raw grayscale data
- `zeros(width, height)` - Create a zero-initialized matrix
- `get_pixel(x, y)` - Get pixel value at a location
- `set_pixel(x, y, value)` - Set pixel value at a location
- `width()`, `height()`, `dimensions()` - Get matrix dimensions
- `data()`, `data_mut()` - Access raw pixel data
- `convolve(kernel, border_mode)` - Apply 2D convolution
- `convolve_separable(kernel_x, kernel_y, border_mode)` - Apply separable convolution
- `resize(width, height, method)` - Resize image with interpolation
- `crop(x, y, width, height)` - Crop image to specified region
- `rotate(angle)` - Rotate image by 90, 180, or 270 degrees (fast, lossless)
- `rotate_custom(angle, method)` - Rotate image by arbitrary angle with interpolation

**Note:** Matrix3 has the same convolution methods, which apply the kernel independently to each RGB channel.

### `Kernel`

A 2D convolution kernel for image filtering operations.

**Built-in Kernels:**
- `Kernel::box_blur(size)` - Uniform averaging filter
- `Kernel::gaussian(size, sigma)` - Gaussian blur filter
- `Kernel::sobel_x()` - Horizontal edge detection (3x3)
- `Kernel::sobel_y()` - Vertical edge detection (3x3)
- `Kernel::laplacian()` - Edge detection (3x3)
- `Kernel::sharpen()` - Sharpening filter (3x3)
- `Kernel::new(width, height, data)` - Custom kernel

**Example:**
```rust
// Create a Gaussian blur kernel
let kernel = Kernel::gaussian(5, 1.0);

// Create a custom kernel
let emboss = Kernel::new(3, 3, vec![
    -2.0, -1.0,  0.0,
    -1.0,  1.0,  1.0,
     0.0,  1.0,  2.0,
]);
```

### `BorderMode`

Specifies how to handle pixels outside the image boundaries during convolution.

**Modes:**
- `BorderMode::Zero` - Pad with zeros
- `BorderMode::Replicate` - Replicate edge pixels (recommended)
- `BorderMode::Reflect` - Reflect across the edge
- `BorderMode::Wrap` - Wrap around to opposite edge

### `InterpolationMethod`

Interpolation method for resizing operations.

**Methods:**
- `InterpolationMethod::NearestNeighbor` - Fastest, lowest quality (good for pixel art)
- `InterpolationMethod::Bilinear` - Good balance of speed and quality (recommended)

### `RotationAngle`

Rotation angle in 90-degree increments (fast, lossless).

**Angles:**
- `RotationAngle::Rotate90` - Rotate 90 degrees clockwise
- `RotationAngle::Rotate180` - Rotate 180 degrees
- `RotationAngle::Rotate270` - Rotate 270 degrees clockwise (90 degrees counter-clockwise)

### `Rotation`

Arbitrary rotation angle for custom rotation with interpolation.

**Units:**
- `Rotation::Degrees(angle)` - Rotation in degrees (e.g., `Rotation::Degrees(45.0)`)
- `Rotation::Radians(angle)` - Rotation in radians (e.g., `Rotation::Radians(PI / 4.0)`)

**Methods:**
- `to_radians()` - Convert to radians
- `to_degrees()` - Convert to degrees

**Examples:**
```rust
// Rotate 45 degrees clockwise
let rotated = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);

// Rotate PI/6 radians (30 degrees)
let rotated = image.rotate_custom(Rotation::Radians(std::f32::consts::PI / 6.0), InterpolationMethod::Bilinear);

// Counter-clockwise rotation with negative angle
let rotated = image.rotate_custom(Rotation::Degrees(-22.5), InterpolationMethod::Bilinear);
```

### Color Space Conversion Functions

**RGB ↔ HSV:**
- `rgb_to_hsv(r, g, b)` - Convert RGB (0-255) to HSV (H: 0-360°, S/V: 0.0-1.0)
- `hsv_to_rgb(h, s, v)` - Convert HSV to RGB

**RGB ↔ HSL:**
- `rgb_to_hsl(r, g, b)` - Convert RGB (0-255) to HSL (H: 0-360°, S/L: 0.0-1.0)
- `hsl_to_rgb(h, s, l)` - Convert HSL to RGB

**Grayscale Methods:**
- `GrayscaleMethod::Luminosity` - Weighted average: 0.299*R + 0.587*G + 0.114*B
- `GrayscaleMethod::Average` - Simple average: (R + G + B) / 3
- `GrayscaleMethod::Lightness` - Midpoint: (max(R,G,B) + min(R,G,B)) / 2

### `io::read_jpeg(path)`

Reads a JPEG image file and returns it as a three-channel RGB `Matrix3`.

**Supported Formats:**
- RGB24 (24-bit color)
- L8 (8-bit grayscale, converted to RGB)
- CMYK32 (CMYK color space, converted to RGB)

**Returns:** `Result<Matrix3, ImageError>`

### `io::read_png(path)`

Reads a PNG image file and returns it as a three-channel RGB `Matrix3`.

**Supported Formats:**
- RGB (24-bit color)
- RGBA (32-bit color with alpha, alpha channel stripped)
- Grayscale (8-bit, converted to RGB)
- Grayscale+Alpha (16-bit, alpha channel stripped, converted to RGB)

**Returns:** `Result<Matrix3, ImageError>`

### `io::write_jpeg(matrix, path, quality)`

Writes a `Matrix3` as a JPEG image file.

**Arguments:**
- `matrix` - Reference to the Matrix3 containing RGB data
- `path` - Path where the JPEG file should be written
- `quality` - JPEG quality (1-100, where 100 is best quality)

**Returns:** `Result<(), ImageError>`

**Example:**
```rust
let image = Matrix3::zeros(640, 480);
write_jpeg(&image, "output.jpg", 90)?;
```

### `io::write_png(matrix, path)`

Writes a `Matrix3` as a PNG image file (lossless compression).

**Arguments:**
- `matrix` - Reference to the Matrix3 containing RGB data
- `path` - Path where the PNG file should be written

**Returns:** `Result<(), ImageError>`

**Example:**
```rust
let image = Matrix3::zeros(640, 480);
write_png(&image, "output.png")?;
```

## Use Cases

### Embedded Systems
- Camera processing on microcontrollers
- Real-time video processing on FPGA/ASIC
- LCD/OLED display controllers
- Industrial vision systems

### High-Performance Applications
- Real-time computer vision
- Video processing pipelines
- Live streaming applications
- Robotics and automation

## Performance

The convolution implementation is highly optimized:

- **Parallel Processing**: When the `parallel` feature is enabled (default), convolution operations automatically use all available CPU cores via Rayon
- **Separable Convolution**: For separable kernels (like Gaussian blur), use `convolve_separable()` which reduces complexity from O(n²) to O(2n), providing significant speedup for large kernels
- **Cache-Friendly**: Row-major memory layout optimized for CPU cache efficiency
- **Zero-Copy**: Direct pixel access without unnecessary allocations

**Benchmark Results** (example on 1920x1080 image):
- 3x3 kernel: ~5-10ms (with parallel)
- 5x5 Gaussian: ~15-20ms (2D), ~8-12ms (separable)
- 9x9 Gaussian: ~50-70ms (2D), ~15-20ms (separable)
- 15x15 Gaussian: ~150-200ms (2D), ~25-35ms (separable)

*Performance varies based on hardware and parallel processing settings.*

## Roadmap

- [x] `no_std` support
- [x] JPEG image reading (with `std` feature)
- [x] PNG image reading (with `std` feature)
- [x] JPEG image writing (with `std` feature)
- [x] PNG image writing (with `std` feature)
- [x] Color space conversions (RGB ↔ HSV, RGB ↔ HSL)
- [x] RGB to Grayscale conversion (multiple methods)
- [x] Single-channel matrix (Matrix1) for grayscale images
- [x] 2D convolution operations with multiple border modes
- [x] Separable convolution for efficiency
- [x] Parallel processing support with Rayon
- [x] Built-in convolution kernels (Gaussian, Sobel, Laplacian, etc.)
- [x] Basic image operations (resize, crop, rotate)
- [ ] Additional color space conversions (RGB ↔ YUV, YCbCr)
- [ ] Morphological operations (erosion, dilation)</parameter>
- [ ] Feature detection
- [ ] SIMD optimizations

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
