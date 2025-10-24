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

## Troubleshooting

### "Failed to read image"
- Make sure `input.jpg` or `input.png` exists in the examples directory
- Check file permissions
- Verify the image format is valid

### Slow performance
- Use `--release` mode
- Ensure `parallel` feature is enabled (default)
- Use separable convolution for large kernels

### Out of memory
- Reduce image size
- Use smaller kernels
- Process images in tiles (implement your own tiling logic)