# Image Writing Functionality

This document describes the image writing capabilities added to cv-rusty, allowing you to save `Matrix3` objects as JPEG or PNG files.

## Overview

The library now supports writing RGB image data from `Matrix3` objects to disk in two popular formats:

- **JPEG** - Lossy compression with adjustable quality
- **PNG** - Lossless compression

## API Reference

### Writing JPEG Images

```rust
pub fn write_jpeg<P: AsRef<Path>>(
    matrix: &Matrix3,
    path: P,
    quality: u8,
) -> Result<(), ImageError>
```

**Parameters:**

- `matrix`: Reference to a `Matrix3` containing RGB data
- `path`: File path where the JPEG should be saved
- `quality`: Compression quality (1-100)

  - `1`: Lowest quality, smallest file size
  - `100`: Highest quality, largest file size
  - `90`: Recommended for most use cases
  - Values outside 1-100 are automatically clamped

**Example:**
```rust
use cv_rusty::{Matrix3, write_jpeg};

let image = Matrix3::zeros(640, 480);
// ... populate image data ...

// Write high-quality JPEG
write_jpeg(&image, "output.jpg", 95)?;

// Write smaller file with lower quality
write_jpeg(&image, "compressed.jpg", 60)?;
```

### Writing PNG Images

```rust
pub fn write_png<P: AsRef<Path>>(
    matrix: &Matrix3,
    path: P,
) -> Result<(), ImageError>
```

**Parameters:**
- `matrix`: Reference to a `Matrix3` containing RGB data
- `path`: File path where the PNG should be saved

**Example:**
```rust
use cv_rusty::{Matrix3, write_png};

let image = Matrix3::zeros(640, 480);
// ... populate image data ...

// Write lossless PNG
write_png(&image, "output.png")?;
```

## Format Comparison

### JPEG
**Pros:**

- Smaller file sizes
- Adjustable quality/size tradeoff
- Widely supported
- Good for photographs

**Cons:**

- Lossy compression (data loss)
- Not suitable for text or sharp edges
- No transparency support

**Typical file sizes (640×480 image):**

- Quality 95: ~35 KB
- Quality 75: ~11 KB
- Quality 50: ~6 KB

### PNG
**Pros:**

- Lossless compression (no data loss)
- Perfect for graphics with sharp edges
- Supports transparency (though cv-rusty uses RGB without alpha)
- Good for iterative editing

**Cons:**

- Larger file sizes than JPEG
- Less efficient for photographs

**Typical file sizes (640×480 image):**

- RGB gradient: ~240 KB

## Complete Workflow Example

```rust
use cv_rusty::{read_jpeg, write_jpeg, write_png, Matrix3};

fn process_and_save() -> Result<(), Box<dyn std::error::Error>> {
    // Read an image
    let mut image = read_jpeg("input.jpg")?;

    // Process the image (example: invert colors)
    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                image.set_pixel(x, y, 255 - r, 255 - g, 255 - b);
            }
        }
    }

    // Save in multiple formats
    write_jpeg(&image, "output_high_quality.jpg", 95)?;
    write_jpeg(&image, "output_web_quality.jpg", 80)?;
    write_png(&image, "output_lossless.png")?;

    Ok(())
}
```

## Error Handling

Both functions return `Result<(), ImageError>` which can have the following error types:

- `ImageError::Io`: File system errors (permissions, disk space, etc.)
- `ImageError::JpegEncode`: JPEG encoding errors
- `ImageError::PngEncode`: PNG encoding errors

```rust
use cv_rusty::io::{write_jpeg, ImageError};

match write_jpeg(&image, "output.jpg", 90) {
    Ok(_) => println!("Image saved successfully"),
    Err(ImageError::Io(e)) => eprintln!("I/O error: {}", e),
    Err(ImageError::JpegEncode(e)) => eprintln!("Encoding error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Quality Guidelines

### JPEG Quality Recommendations

| Quality | Use Case | File Size | Visual Quality |
|---------|----------|-----------|----------------|
| 90-100  | Archival, printing, professional use | Large | Excellent |
| 80-89   | High-quality web images | Medium | Very Good |
| 60-79   | Standard web images, general use | Small | Good |
| 40-59   | Thumbnails, low-bandwidth scenarios | Very Small | Acceptable |
| 1-39    | Extreme compression | Minimal | Poor |

### When to Use Each Format

**Use JPEG when:**

- Working with photographs
- File size is a concern
- Minor quality loss is acceptable
- Sharing on the web

**Use PNG when:**

- Preserving exact pixel values is critical
- Working with graphics, text, or line art
- Need to perform multiple save operations
- File size is not a primary concern

## Dependencies

The image writing functionality requires the `std` feature (enabled by default) and uses:

- `jpeg-encoder` (v0.6) for JPEG encoding
- `png` (v0.17) for PNG encoding

These dependencies are automatically included when using the default `std` feature.

## Examples

Run the included examples to see the functionality in action:

```bash
# Create gradient images in both formats
cargo run --example write_image_example

# Convert an existing image to different formats
cargo run --example image_conversion input.jpg

# View all examples
ls examples/
```

## Performance Considerations

- **JPEG encoding** is generally faster than PNG for photographs
- **PNG encoding** time increases with image complexity
- Both encoders are optimized for single-threaded operation
- Memory usage is proportional to image size: width × height × 3 bytes for RGB data

## Future Enhancements

Potential improvements for future versions:
- Support for writing grayscale images
- PNG transparency (RGBA) support
- Progressive JPEG encoding
- Configurable PNG compression levels
- Batch processing utilities
- SIMD-optimized encoding

## See Also

- [README.md](README.md) - General library documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - Library architecture
- [API Documentation](src/io.rs) - Full API reference
