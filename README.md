# CV Rusty

A `no_std` computer vision library written in Rust, designed for live computations, embedded systems, and high-performance image processing.

## Features

- **`no_std` Compatible**: Core library works without the standard library (only requires `alloc`)
- **Zero-copy Image Representation**: Efficient three-channel matrix structure for RGB images
- **JPEG Reading**: Built-in support for reading JPEG images with automatic format conversion (requires `std` feature)
- **Format Support**: Handles RGB24, Grayscale (L8), and CMYK32 JPEG formats
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

### Reading a JPEG Image (requires `std` feature)

```rust
use cv_rusty::io::read_jpeg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read a JPEG file into a Matrix3
    let image = read_jpeg("photo.jpg")?;
    
    println!("Image dimensions: {}x{}", image.width(), image.height());
    
    // Access pixel data
    if let Some((r, g, b)) = image.get_pixel(100, 100) {
        println!("Pixel at (100, 100): RGB({}, {}, {})", r, g, b);
    }
    
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
use cv_rusty::io::{read_jpeg, ImageError};

match read_jpeg("photo.jpg") {
    Ok(image) => {
        println!("Successfully loaded {}x{} image", image.width(), image.height());
    }
    Err(ImageError::Io(e)) => {
        eprintln!("File I/O error: {}", e);
    }
    Err(ImageError::JpegDecode(e)) => {
        eprintln!("JPEG decoding error: {}", e);
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
- [ ] PNG image support
- [ ] Image writing capabilities
- [ ] Color space conversions (RGB ↔ HSV, YUV)
- [ ] Basic image operations (resize, crop, rotate)
- [ ] Filtering and convolution
- [ ] Edge detection
- [ ] Feature detection
- [ ] SIMD optimizations

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.