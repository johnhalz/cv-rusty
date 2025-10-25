# Custom Rotation Feature Guide

This document provides a comprehensive guide to the custom rotation feature added to cv-rusty, which supports arbitrary rotation angles with both degrees and radians.

## Overview

The custom rotation feature extends cv-rusty's rotation capabilities beyond the original 90-degree increments to support any arbitrary angle. This enables precise image adjustments, straightening tilted images, and creating artistic effects.

## Key Features

- ✅ **Arbitrary Angles**: Rotate by any angle, not just 90° increments
- ✅ **Dual Units**: Support for both degrees and radians
- ✅ **Bidirectional**: Positive values rotate clockwise, negative counter-clockwise
- ✅ **Interpolation Options**: Choose between nearest neighbor (fast) or bilinear (quality)
- ✅ **Auto-sizing**: Output dimensions automatically calculated to fit entire rotated image
- ✅ **no_std Compatible**: Works in embedded environments
- ✅ **Both Matrix Types**: Available for Matrix3 (RGB) and Matrix1 (grayscale)

## API Reference

### Rotation Enum

```rust
pub enum Rotation {
    /// Rotation in degrees (e.g., 45.0 for 45 degrees clockwise)
    Degrees(f32),
    /// Rotation in radians (e.g., PI/4 for 45 degrees clockwise)
    Radians(f32),
}
```

### Methods

```rust
impl Rotation {
    /// Converts the rotation to radians
    pub fn to_radians(&self) -> f32
    
    /// Converts the rotation to degrees
    pub fn to_degrees(&self) -> f32
}
```

### Matrix Methods

```rust
impl Matrix3 {
    /// Rotates the image by an arbitrary angle using interpolation
    pub fn rotate_custom(&self, angle: Rotation, method: InterpolationMethod) -> Self
}

impl Matrix1 {
    /// Rotates the image by an arbitrary angle using interpolation
    pub fn rotate_custom(&self, angle: Rotation, method: InterpolationMethod) -> Self
}
```

## Usage Examples

### Basic Usage with Degrees

```rust
use cv_rusty::{Matrix3, Rotation, InterpolationMethod};

let image = Matrix3::zeros(640, 480);

// Rotate 45 degrees clockwise
let rotated = image.rotate_custom(
    Rotation::Degrees(45.0),
    InterpolationMethod::Bilinear
);

println!("New dimensions: {}x{}", rotated.width(), rotated.height());
```

### Basic Usage with Radians

```rust
use cv_rusty::{Matrix3, Rotation, InterpolationMethod};

let image = Matrix3::zeros(640, 480);

// Rotate PI/4 radians (45 degrees)
let rotated = image.rotate_custom(
    Rotation::Radians(std::f32::consts::PI / 4.0),
    InterpolationMethod::Bilinear
);
```

### Counter-Clockwise Rotation

```rust
// Use negative angles to rotate counter-clockwise
let rotated_ccw = image.rotate_custom(
    Rotation::Degrees(-30.0),
    InterpolationMethod::Bilinear
);

// Also works with radians
let rotated_ccw = image.rotate_custom(
    Rotation::Radians(-std::f32::consts::PI / 6.0),
    InterpolationMethod::Bilinear
);
```

### Choosing Interpolation Method

```rust
// Bilinear: Better quality, slightly slower
let high_quality = image.rotate_custom(
    Rotation::Degrees(15.0),
    InterpolationMethod::Bilinear
);

// Nearest Neighbor: Faster, more pixelated
let fast = image.rotate_custom(
    Rotation::Degrees(15.0),
    InterpolationMethod::NearestNeighbor
);
```

### Converting Between Units

```rust
use cv_rusty::Rotation;

// Create rotation in degrees
let deg_45 = Rotation::Degrees(45.0);
println!("In radians: {}", deg_45.to_radians()); // 0.785398...

// Create rotation in radians
let rad_pi4 = Rotation::Radians(std::f32::consts::PI / 4.0);
println!("In degrees: {}", rad_pi4.to_degrees()); // 45.0
```

## Common Use Cases

### 1. Straightening Tilted Images

```rust
fn straighten_image(image: &Matrix3, tilt_angle: f32) -> Matrix3 {
    // Negative angle to correct the tilt
    image.rotate_custom(
        Rotation::Degrees(-tilt_angle),
        InterpolationMethod::Bilinear
    )
}

// Example: straighten a photo that's 2.3 degrees tilted
let tilted = read_jpeg("tilted_photo.jpg")?;
let straightened = straighten_image(&tilted, 2.3);
write_jpeg(&straightened, "straightened.jpg", 90)?;
```

### 2. Creating Rotated Thumbnails

```rust
fn create_rotated_thumbnail(image: &Matrix3, angle: f32, size: usize) -> Matrix3 {
    image
        .resize(size, size, InterpolationMethod::Bilinear)
        .rotate_custom(
            Rotation::Degrees(angle),
            InterpolationMethod::Bilinear
        )
}

let thumbnail = create_rotated_thumbnail(&image, 15.0, 200);
```

### 3. Artistic Effects - Multiple Rotations

```rust
fn create_kaleidoscope_effect(image: &Matrix3, divisions: usize) -> Vec<Matrix3> {
    let angle_step = 360.0 / divisions as f32;
    
    (0..divisions)
        .map(|i| {
            let angle = i as f32 * angle_step;
            image.rotate_custom(
                Rotation::Degrees(angle),
                InterpolationMethod::Bilinear
            )
        })
        .collect()
}

// Create 8 rotated versions at different angles
let rotations = create_kaleidoscope_effect(&image, 8);
```

### 4. Precise Alignment

```rust
fn align_images(base: &Matrix3, overlay: &Matrix3, alignment_angle: f32) -> Matrix3 {
    // Rotate overlay to match base image orientation
    overlay.rotate_custom(
        Rotation::Degrees(alignment_angle),
        InterpolationMethod::Bilinear
    )
}
```

### 5. Batch Processing with Different Angles

```rust
fn process_with_angles(image: &Matrix3, angles: &[f32]) -> Vec<Matrix3> {
    angles.iter()
        .map(|&angle| {
            image.rotate_custom(
                Rotation::Degrees(angle),
                InterpolationMethod::Bilinear
            )
        })
        .collect()
}

let angles = vec![15.0, 30.0, 45.0, 60.0, 75.0];
let rotated_images = process_with_angles(&image, &angles);
```

## Performance Considerations

### Interpolation Method Impact

| Method | Speed | Quality | Best For |
|--------|-------|---------|----------|
| Nearest Neighbor | ~2x faster | Lower | Speed-critical, pixel art |
| Bilinear | Baseline | Higher | Photos, general use |

### Angle Impact

- Small angles (< 15°) are fastest
- 45° angles are moderate
- Output image size increases with angle
- Maximum size occurs at 45° for square images

### Memory Usage

```rust
// Calculate memory requirements
let original_size = width * height * 3; // RGB image

// Worst case (45° rotation of square image):
let max_size = (width as f32 * 1.414) as usize; // sqrt(2) ≈ 1.414
let rotated_size = max_size * max_size * 3;

let total_memory = original_size + rotated_size; // Both exist during operation
```

### Optimization Tips

1. **Crop before rotating** - Reduce input size first
2. **Choose appropriate interpolation** - Use nearest neighbor when quality isn't critical
3. **Avoid repeated rotations** - Rotate once to final angle
4. **Consider 90° increments** - Use `rotate()` instead when possible (faster, lossless)

## Output Dimensions

The output image is automatically sized to contain the entire rotated image without cropping.

### Example Dimension Changes

**Square Images (100×100):**
- 0°: 100×100
- 15°: ~104×104
- 30°: ~116×116
- 45°: ~141×141 (maximum)
- 90°: 100×100

**Rectangular Images (200×100):**
- 0°: 200×100
- 30°: ~249×230
- 45°: ~212×212
- 90°: 100×200

## Comparison: Fixed vs Custom Rotation

| Feature | `rotate()` | `rotate_custom()` |
|---------|-----------|-------------------|
| **Angles** | 90°, 180°, 270° | Any angle |
| **Speed** | Very fast | Moderate |
| **Quality** | Lossless | Depends on interpolation |
| **Dimensions** | Predictable | Auto-calculated |
| **Use Case** | Simple rotations | Precise adjustments |
| **Overhead** | None | Interpolation required |

## When to Use Each Method

### Use `rotate()` (Fixed Angles) When:
- You need 90°, 180°, or 270° rotation
- Performance is critical
- You want lossless transformation
- Dimensions are important to predict

### Use `rotate_custom()` (Arbitrary Angles) When:
- You need precise angle control
- Straightening tilted images
- Creating artistic effects
- Aligning images
- Small adjustments (< 45°)

## Error Handling

Both methods always succeed and return a valid image:

```rust
// No Result type - always succeeds
let rotated = image.rotate_custom(
    Rotation::Degrees(45.0),
    InterpolationMethod::Bilinear
);
```

Pixels outside the original image bounds are filled with black (0, 0, 0).

## no_std Compatibility

The custom rotation feature is fully compatible with `no_std` environments:

```rust
#![no_std]
extern crate alloc;

use cv_rusty::{Matrix3, Rotation, InterpolationMethod};

// Works in embedded systems
let image = Matrix3::zeros(320, 240);
let rotated = image.rotate_custom(
    Rotation::Degrees(30.0),
    InterpolationMethod::NearestNeighbor
);
```

Uses `libm` for trigonometric functions in `no_std` mode.

## Testing

Run the custom rotation tests:

```bash
# Run all transform tests
cargo test transform::

# Run specific rotation tests
cargo test rotate_custom

# Test with all features
cargo test --all-features

# Verify no_std compatibility
cargo build --no-default-features
```

## Example Program

See `examples/transform_demo.rs` for a complete demonstration:

```bash
cargo run --example transform_demo --release
```

This will create several output files demonstrating:
- 45° rotation with degrees
- 30° rotation with nearest neighbor
- PI/6 radians rotation
- -22.5° counter-clockwise rotation

## Mathematical Background

### Rotation Matrix

For angle θ (in radians), clockwise rotation:

```
[cos(θ)  -sin(θ)]
[sin(θ)   cos(θ)]
```

### Implementation

Uses inverse rotation to map output pixels to source:

```
For each output pixel (x_out, y_out):
  1. Translate to origin (subtract center)
  2. Apply inverse rotation
  3. Translate back (add center)
  4. Sample source image with interpolation
```

### Why Inverse Rotation?

Forward rotation would leave gaps. Inverse rotation ensures every output pixel is filled by sampling the appropriate source location.

## Limitations

1. **No arbitrary center**: Rotation is always around the image center
2. **No crop to original size**: Output is always sized to fit the full rotation
3. **Quality loss**: Unlike 90° rotations, interpolation causes some quality loss
4. **Performance**: Slower than fixed-angle rotations

## Future Enhancements

Potential additions:
- [ ] Rotation around custom pivot point
- [ ] Crop to original dimensions option
- [ ] Bicubic interpolation
- [ ] SIMD optimizations
- [ ] GPU acceleration

## Additional Resources

- Main documentation: `README.md`
- Transform operations guide: `TRANSFORM_OPERATIONS.md`
- Quick reference: `docs/quick-reference.md`
- API docs: Run `cargo doc --open`

## Questions?

For more information:
1. Check the examples in `examples/transform_demo.rs`
2. Read the API documentation: `cargo doc --open`
3. Look at the test cases in `src/transform.rs`
