# CV Rusty Examples

This directory contains examples demonstrating various features of the cv-rusty library.

## Prerequisites

Most examples require a test image. Place a JPEG or PNG image in this directory as `input.jpg` or `input.png`.

You can download a test image:
```bash
curl -o examples/input.jpg https://upload.wikimedia.org/wikipedia/commons/thumb/3/3a/Cat03.jpg/1200px-Cat03.jpg
```

## Examples

### Convolution Operations

#### Convolution Demo
Demonstrates various convolution filters including Gaussian blur, edge detection, sharpening, and custom kernels.

```bash
cargo run --release --example convolution_demo
```

This will create multiple output files:
- `output_gaussian.jpg` - Gaussian blur (5x5)
- `output_gaussian_sep.jpg` - Separable Gaussian blur (more efficient)
- `output_box.jpg` - Box blur
- `output_sobel_x.jpg` - Horizontal edge detection
- `output_sobel_y.jpg` - Vertical edge detection
- `output_sobel.jpg` - Combined edge magnitude
- `output_laplacian.jpg` - Laplacian edge detection
- `output_sharpen.jpg` - Sharpened image
- `output_emboss.jpg` - Emboss effect
- `output_border_*.jpg` - Different border mode comparisons

#### Convolution Benchmark
Compares performance of different kernel sizes and convolution methods.

```bash
# With parallel processing (default)
cargo run --release --example convolution_benchmark

# Without parallel processing (sequential)
cargo run --release --example convolution_benchmark --no-default-features --features std
```

This benchmark will show:
- Performance of small (3x3), medium (5x5), large (9x9), and very large (15x15) kernels
- Comparison between 2D and separable convolution
- Border mode performance comparison
- Throughput in megapixels per second

### Color Space Conversions

#### Color Conversion Example
Demonstrates RGB to HSV, HSL conversions and grayscale conversion methods.

```bash
cargo run --example color_conversion_example
```

### Image I/O

#### Image Conversion
Converts between JPEG and PNG formats.

```bash
cargo run --example image_conversion path/to/image.jpg
```

#### Read JPEG Example
Reads and displays information about a JPEG image.

```bash
cargo run --example read_jpeg_example path/to/image.jpg
```

#### Read PNG Example
Reads and displays information about a PNG image.

```bash
cargo run --example read_png_example path/to/image.png
```

#### Write Image Example
Creates test patterns and writes them as JPEG and PNG.

```bash
cargo run --example write_image_example
```

### Drawing Shapes

#### Drawing Example
Demonstrates drawing rectangles and circles on RGB images with various styles, colors, rotations, and stroke widths.

```bash
cargo run --example drawing_example
```

This example creates a canvas and draws:
- Filled rectangles with borders (with and without rotation)
- Filled circles with borders
- Outline-only shapes (no fill)
- Overlapping shapes

Output: `drawing_output.png`

#### Drawing Grayscale Example
Demonstrates drawing on grayscale (Matrix1) images.

```bash
cargo run --example drawing_grayscale_example
```

Output: `drawing_grayscale_output.png`

#### Drawing Hex Colors Example
Demonstrates using hex color strings for drawing.

```bash
cargo run --example drawing_hex_colors
```

This example shows:
- 6-digit hex format (`#RRGGBB` or `RRGGBB`)
- 3-digit hex format (`#RGB` or `RGB`)
- Using `Color::from_hex()` method
- Using `.parse()` with `FromStr` trait
- Common web colors

Output: `drawing_hex_colors.png`

### Image Transformations

#### Transform Demo
Demonstrates image rotation and transformation operations.

```bash
cargo run --example transform_demo
```

### Window Display (GUI)

#### Simple show_image
Demonstrates basic image display in a window.

```bash
cargo run --example simple_show_image --features window
```

This example creates a simple test pattern with a red square and blue border, then displays it in a window. Press ESC or close the window to exit.

#### Window Display Example
Comprehensive example showing various window display capabilities:
- Color gradient images
- Grayscale radial gradients
- Checkerboard patterns
- Loading and displaying images from files

```bash
cargo run --example window_display_example --features window
```

**Note**: The window feature requires GUI support and is not available in headless environments.

### No-std Example

#### No-std Example
Demonstrates core functionality without the standard library.

```bash
cargo run --example no_std_example
```

## Performance Tips

1. **Always use `--release` mode** for performance testing:
   ```bash
   cargo run --release --example convolution_benchmark
   ```

2. **Use separable convolution** for large kernels (especially Gaussian blur):
   - 2D convolution: O(n²) operations per pixel
   - Separable: O(2n) operations per pixel
   - For a 9x9 kernel: ~4.5x faster
   - For a 15x15 kernel: ~7.5x faster

3. **Parallel processing** is enabled by default:
   - Automatically uses all CPU cores
   - Disable with: `--no-default-features --features std`
   - Best for large images and kernels

4. **Border modes** have similar performance, use the one that fits your use case:
   - `BorderMode::Replicate` - Recommended for most cases
   - `BorderMode::Zero` - Fastest, but may create edge artifacts
   - `BorderMode::Reflect` - Good for seamless tiling
   - `BorderMode::Wrap` - For periodic patterns

## Example Output

The convolution examples will demonstrate various effects:

- **Gaussian Blur**: Smooth blur effect, good for noise reduction
- **Box Blur**: Uniform blur, faster but less smooth than Gaussian
- **Sobel X/Y**: Detect horizontal/vertical edges
- **Laplacian**: Detect all edges (omnidirectional)
- **Sharpen**: Enhance image details and edges
- **Emboss**: 3D embossed effect

## Creating Your Own Examples

### Convolution Example
```rust
use cv_rusty::{read_jpeg, write_jpeg, show_image, Kernel, BorderMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let image = read_jpeg("input.jpg")?;
    
    // Display original
    show_image("Original", &image)?;
    
    // Apply Gaussian blur
    let kernel = Kernel::gaussian(5, 1.0);
    let blurred = image.convolve(&kernel, BorderMode::Replicate);
    
    // Display and save result
    show_image("Blurred", &blurred)?;
    write_jpeg(&blurred, "output.jpg", 90)?;
    
    Ok(())
}
```

### Drawing Example
```rust
use cv_rusty::{Matrix3, draw_rectangle, draw_circle, write_png, Color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a white canvas
    let mut image = Matrix3::zeros(480, 640);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }
    
    // Draw a red rectangle with black border
    draw_rectangle(
        &mut image,
        320.0, 240.0,  // center position (x, y)
        100.0, 80.0,   // width, height
        45.0,          // rotation in degrees
        3,             // stroke width
        Some(Color::rgb(0, 0, 0)),       // black stroke
        Some(Color::rgb(255, 0, 0))      // red fill
    );
    
    // Use hex colors
    draw_rectangle(
        &mut image,
        420.0, 240.0,
        100.0, 80.0,
        0.0,
        2,
        Some(Color::from_hex("#2C3E50")?),  // dark blue-gray
        Some(Color::from_hex("#3498DB")?)   // light blue
    );
    
    // Draw a blue circle with white border
    draw_circle(
        &mut image,
        200.0, 200.0,  // center position (x, y)
        50.0,          // radius
        2,             // stroke width
        Some(Color::rgb(255, 255, 255)), // white stroke
        Some(Color::rgb(0, 0, 255))      // blue fill
    );
    
    // Parse color from string
    let pink: Color = "#FF1493".parse()?;
    draw_circle(&mut image, 300.0, 200.0, 40.0, 2,
                Some(Color::black()), Some(pink));
    
    // Save result
    write_png(&image, "output.png")?;
    
    Ok(())
}
```

## Features

The library supports several optional features:

- `std` (default): Standard library support with file I/O
- `parallel` (default): Parallel processing using rayon
- `window`: GUI window support for displaying images

To use specific features:
```bash
# With window display
cargo run --example simple_show_image --features window

# Without parallel processing
cargo run --example convolution_demo --no-default-features --features std

# All features
cargo run --example window_display_example --all-features
```

## Troubleshooting

### "Failed to read image"
- Make sure `input.jpg` or `input.png` exists in the examples directory
- Check file permissions
- Verify the image format is valid

### "This example requires the 'window' feature"
- Add `--features window` to your cargo command
- Example: `cargo run --example simple_imshow --features window`
- Window feature requires GUI support (not available in headless environments)

### Slow performance
- Use `--release` mode
- Ensure `parallel` feature is enabled (default)
- Use separable convolution for large kernels

### Out of memory
- Reduce image size
- Use smaller kernels
- Process images in tiles (implement your own tiling logic)