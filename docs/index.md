# CV Rusty

A `no_std` computer vision library written in Rust, designed for live computations, embedded systems, and high-performance image processing.

## Features

- **`no_std` Compatible**: Core library works without the standard library (only requires `alloc`)
- **Zero-copy Image Representation**: Efficient three-channel matrix structure for RGB images
- **Image I/O**: Built-in support for reading and writing JPEG and PNG images with automatic format conversion (requires `std` feature)
- **Format Support**: Handles RGB24, Grayscale (L8), and CMYK32 JPEG formats; RGB, RGBA, Grayscale, and Grayscale+Alpha PNG formats
- **Safe API**: Bounds-checked pixel access with ergonomic error handling
- **Embedded Ready**: Perfect for resource-constrained environments and real-time systems

## Installation

### Standard Library (default)

For applications with `std` support and file I/O:

```toml
[dependencies]
cv-rusty = "0.1.0"
```

### `no_std` Environments

For embedded systems or `no_std` environments (requires `alloc`):

```toml
[dependencies]
cv-rusty = { version = "0.1.0", default-features = false }
```

## Feature Flags

- **`std`** (enabled by default): Enables standard library support, including file I/O operations
- **`alloc`**: Enables heap allocation support (required for core functionality)

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
```

### Working with Matrix3 (`no_std` compatible)

```rust
use cv_rusty::matrix::Matrix3;

// Create a new 640x480 image filled with zeros
let mut image = Matrix3::zeros(640, 480);

// Set a pixel value
image.set_pixel(10, 20, 255, 0, 0); // Red pixel at (10, 20)

// Get a pixel value
if let Some((r, g, b)) = image.get_pixel(10, 20) {
    println!("RGB: ({}, {}, {})", r, g, b);
}

// Access raw data
let raw_data = image.data();
println!("Total bytes: {}", raw_data.len());
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
- [ ] Morphological operations (erosion, dilation)
- [ ] Feature detection
- [ ] SIMD optimizations

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
