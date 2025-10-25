# Image Transform Operations

This document provides an overview of the image transformation operations added to cv-rusty.

## Overview

The transform module provides three core operations for image manipulation:
- **Resize**: Scale images up or down with different interpolation methods
- **Crop**: Extract rectangular regions from images
- **Rotate**: Rotate images in 90-degree increments (fast, lossless) or by arbitrary angles with interpolation

All operations are `no_std` compatible and work with both `Matrix3` (RGB) and `Matrix1` (grayscale) image types.

## Resize

Resize images to specified dimensions with configurable interpolation methods.

### Interpolation Methods

#### Nearest Neighbor (`InterpolationMethod::NearestNeighbor`)
- **Speed**: Fastest
- **Quality**: Lowest
- **Best for**: Pixel art, icons, when speed is critical
- **Algorithm**: Selects the nearest pixel without blending

#### Bilinear (`InterpolationMethod::Bilinear`)
- **Speed**: Moderate
- **Quality**: Good
- **Best for**: Most general-purpose resizing, photos
- **Algorithm**: Linear interpolation in both directions

### Usage

```rust
use cv_rusty::{Matrix3, InterpolationMethod};

let image = Matrix3::zeros(640, 480);

// Downscale with bilinear interpolation
let thumbnail = image.resize(320, 240, InterpolationMethod::Bilinear);

// Upscale with nearest neighbor
let enlarged = image.resize(1280, 960, InterpolationMethod::NearestNeighbor);
```

### Performance Characteristics

- **Nearest Neighbor**: O(width × height) - very fast
- **Bilinear**: O(width × height) - slightly slower due to interpolation calculations
- **Memory**: Creates a new image buffer, original remains unchanged

## Crop

Extract a rectangular region from an image.

### Parameters

- `x`: X-coordinate of top-left corner (0-based)
- `y`: Y-coordinate of top-left corner (0-based)
- `width`: Width of the crop region
- `height`: Height of the crop region

### Usage

```rust
use cv_rusty::Matrix3;

let image = Matrix3::zeros(640, 480);

// Crop a 200x200 region starting at (100, 100)
let cropped = image.crop(100, 100, 200, 200).unwrap();

// Center crop
let (w, h) = image.dimensions();
let crop_size = 300;
let x = (w - crop_size) / 2;
let y = (h - crop_size) / 2;
let center = image.crop(x, y, crop_size, crop_size).unwrap();
```

### Error Handling

The `crop()` method returns `Option<Matrix>`:
- `Some(Matrix)` if the crop region is valid
- `None` if the region extends beyond image boundaries

```rust
match image.crop(x, y, width, height) {
    Some(cropped) => println!("Crop successful"),
    None => println!("Invalid crop region"),
}
```

### Performance Characteristics

- **Time Complexity**: O(width × height) - linear copy operation
- **Memory**: Efficiently copies only the required region
- **Optimization**: Uses `copy_from_slice` for row-wise copying

## Rotate

Rotate images using two methods:
1. **Fixed angles** (90°, 180°, 270°) - Fast, lossless transformation
2. **Arbitrary angles** - Custom rotation with interpolation

### Fixed Angle Rotation (Fast, Lossless)

#### Rotation Angles

- `RotationAngle::Rotate90`: 90 degrees clockwise
- `RotationAngle::Rotate180`: 180 degrees
- `RotationAngle::Rotate270`: 270 degrees clockwise (90 degrees counter-clockwise)

#### Usage

```rust
use cv_rusty::{Matrix3, RotationAngle};

let image = Matrix3::zeros(640, 480);

// Rotate 90 degrees clockwise (output: 480×640)
let rotated = image.rotate(RotationAngle::Rotate90);

// Rotate 180 degrees (output: 640×480)
let flipped = image.rotate(RotationAngle::Rotate180);

// Rotate 270 degrees clockwise (output: 480×640)
let rotated_ccw = image.rotate(RotationAngle::Rotate270);
```

#### Dimension Changes

| Original Size | Rotation | Result Size |
|---------------|----------|-------------|
| 640×480 | 90° | 480×640 |
| 640×480 | 180° | 640×480 |
| 640×480 | 270° | 480×640 |

#### Performance Characteristics

- **Time Complexity**: O(width × height) - single pass pixel remapping
- **Quality**: Lossless - no interpolation needed
- **Memory**: Creates a new image buffer with transposed dimensions for 90°/270° rotations

### Custom Angle Rotation (Arbitrary Angles)

Rotate images by any angle (not just 90-degree increments) using interpolation.

#### Rotation Units

The `Rotation` enum supports both degrees and radians:

- `Rotation::Degrees(angle)`: Specify rotation in degrees
- `Rotation::Radians(angle)`: Specify rotation in radians

**Positive values** rotate clockwise, **negative values** rotate counter-clockwise.

#### Usage with Degrees

```rust
use cv_rusty::{Matrix3, Rotation, InterpolationMethod};

let image = Matrix3::zeros(640, 480);

// Rotate 45 degrees clockwise
let rotated = image.rotate_custom(
    Rotation::Degrees(45.0),
    InterpolationMethod::Bilinear
);

// Rotate 30 degrees with nearest neighbor (faster)
let rotated = image.rotate_custom(
    Rotation::Degrees(30.0),
    InterpolationMethod::NearestNeighbor
);

// Rotate counter-clockwise with negative angle
let rotated = image.rotate_custom(
    Rotation::Degrees(-22.5),
    InterpolationMethod::Bilinear
);
```

#### Usage with Radians

```rust
use cv_rusty::{Matrix3, Rotation, InterpolationMethod};

let image = Matrix3::zeros(640, 480);

// Rotate PI/4 radians (45 degrees)
let rotated = image.rotate_custom(
    Rotation::Radians(std::f32::consts::PI / 4.0),
    InterpolationMethod::Bilinear
);

// Rotate PI/6 radians (30 degrees)
let rotated = image.rotate_custom(
    Rotation::Radians(std::f32::consts::PI / 6.0),
    InterpolationMethod::Bilinear
);

// Negative radians for counter-clockwise
let rotated = image.rotate_custom(
    Rotation::Radians(-std::f32::consts::PI / 8.0),
    InterpolationMethod::Bilinear
);
```

#### Converting Between Units

```rust
let deg_45 = Rotation::Degrees(45.0);
let radians = deg_45.to_radians();  // Returns 0.785398... (PI/4)
let degrees = deg_45.to_degrees();  // Returns 45.0

let rad_pi4 = Rotation::Radians(std::f32::consts::PI / 4.0);
let degrees = rad_pi4.to_degrees(); // Returns 45.0
let radians = rad_pi4.to_radians(); // Returns 0.785398...
```

#### Output Image Size

Custom rotation automatically calculates the output dimensions to contain the entire rotated image without cropping.

**Example**: Rotating a 100×100 image by 45°
- Output size: approximately 141×141 (to fit diagonal)

#### Performance Characteristics

- **Time Complexity**: O(width × height) - processes all output pixels
- **Quality**: Depends on interpolation method
  - Bilinear: Good quality, slight blur
  - Nearest Neighbor: Faster, more pixelated
- **Memory**: Creates new image buffer sized to contain full rotated image

#### Comparison: Fixed vs Custom Rotation

| Feature | Fixed (90°/180°/270°) | Custom (Arbitrary) |
|---------|----------------------|-------------------|
| Speed | Very Fast | Moderate |
| Quality | Lossless | Depends on interpolation |
| Angles | 90° increments only | Any angle |
| Dimensions | Predictable | Calculated dynamically |
| Use Case | Simple rotations | Precise adjustments |

## Chaining Operations

All transform operations return new `Matrix` objects, allowing for easy chaining:

```rust
use cv_rusty::{Matrix3, InterpolationMethod, RotationAngle};

let processed = image
    .crop(50, 50, 500, 400)     // Crop region
    .unwrap()
    .resize(250, 200, InterpolationMethod::Bilinear)  // Resize
    .rotate(RotationAngle::Rotate90);  // Rotate
```

### Recommended Operation Order

For best results, chain operations in this order:

1. **Crop** - Reduce data size first
2. **Rotate** - Apply lossless transformations
3. **Resize** - Final scaling (may introduce interpolation artifacts)

```rust
// Efficient pipeline
let thumbnail = image
    .crop(100, 100, 400, 300)   // 1. Reduce to region of interest
    .unwrap()
    .rotate(RotationAngle::Rotate90)  // 2. Lossless rotation
    .resize(150, 200, InterpolationMethod::Bilinear);  // 3. Final resize
```

## `no_std` Compatibility

All transform operations are fully compatible with `no_std` environments:

```rust
#![no_std]
extern crate alloc;

use cv_rusty::{Matrix3, InterpolationMethod, RotationAngle};

// All operations work in no_std
let image = Matrix3::zeros(320, 240);
let resized = image.resize(160, 120, InterpolationMethod::Bilinear);
let cropped = image.crop(10, 10, 100, 100).unwrap();
let rotated = image.rotate(RotationAngle::Rotate90);
```

### Memory Considerations for Embedded Systems

When working in memory-constrained environments:

```rust
// Calculate memory requirements before operations
let original_size = width * height * 3;  // RGB image
let resized_size = new_width * new_height * 3;
let total_memory = original_size + resized_size;  // Both exist during operation

// Consider dropping original after transform
let resized = {
    let temp = Matrix3::zeros(640, 480);
    temp.resize(320, 240, InterpolationMethod::NearestNeighbor)
    // temp is dropped here, freeing memory
};
```

## Examples

### Create Thumbnails

```rust
fn create_thumbnail(image: &Matrix3, max_size: usize) -> Matrix3 {
    let (w, h) = image.dimensions();
    let scale = max_size as f32 / w.max(h) as f32;
    let new_w = (w as f32 * scale) as usize;
    let new_h = (h as f32 * scale) as usize;
    
    image.resize(new_w, new_h, InterpolationMethod::Bilinear)
}
```

### Center Crop and Resize

```rust
fn center_crop_resize(image: &Matrix3, size: usize) -> Matrix3 {
    let (w, h) = image.dimensions();
    let crop_size = w.min(h);
    let x = (w - crop_size) / 2;
    let y = (h - crop_size) / 2;
    
    image.crop(x, y, crop_size, crop_size)
        .unwrap()
        .resize(size, size, InterpolationMethod::Bilinear)
}
```

### Image Tiling

```rust
fn create_tiles(image: &Matrix3, tile_size: usize) -> Vec<Matrix3> {
    let (w, h) = image.dimensions();
    let mut tiles = Vec::new();
    
    for y in (0..h).step_by(tile_size) {
        for x in (0..w).step_by(tile_size) {
            let tw = tile_size.min(w - x);
            let th = tile_size.min(h - y);
            if let Some(tile) = image.crop(x, y, tw, th) {
                tiles.push(tile);
            }
        }
    }
    
    tiles
}
```

### Batch Processing

```rust
fn batch_resize(images: Vec<Matrix3>, width: usize, height: usize) -> Vec<Matrix3> {
    images.into_iter()
        .map(|img| img.resize(width, height, InterpolationMethod::Bilinear))
        .collect()
}
```

### Rotate and Straighten

```rust
// Rotate image to straighten it based on detected angle
fn straighten_image(image: &Matrix3, angle_degrees: f32) -> Matrix3 {
    // Rotate to correct the angle
    image.rotate_custom(
        Rotation::Degrees(-angle_degrees),
        InterpolationMethod::Bilinear
    )
}

// Example: straighten a slightly tilted scan
let tilted_scan = Matrix3::zeros(640, 480);
let straightened = straighten_image(&tilted_scan, 2.3); // Correct 2.3° tilt
```

### Create Panorama Effect

```rust
// Rotate multiple copies at different angles
fn create_fan_effect(image: &Matrix3, steps: usize) -> Vec<Matrix3> {
    let angle_step = 360.0 / steps as f32;
    (0..steps)
        .map(|i| {
            let angle = i as f32 * angle_step;
            image.rotate_custom(
                Rotation::Degrees(angle),
                InterpolationMethod::Bilinear
            )
        })
        .collect()
}
```

## Testing

The transform module includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run transform-specific tests
cargo test transform::

# Run with all features
cargo test --all-features

# Test no_std compatibility
cargo build --no-default-features
```

## Performance Tips

1. **Choose the right interpolation method**:
   - Use `NearestNeighbor` for pixel art or when speed is critical
   - Use `Bilinear` for photos and general-purpose resizing

2. **Crop before resize**:
   - Process less data by cropping first

3. **Avoid multiple resizes**:
   - Calculate target size once and resize directly

4. **Use rotation for 90° increments**:
   - Much faster and lossless compared to arbitrary angle rotation

5. **Reuse memory when possible**:
   - Drop intermediate results to free memory

## Implementation Details

### Resize Algorithm

**Nearest Neighbor**:
```
for each output pixel (x, y):
    src_x = floor(x * width_ratio)
    src_y = floor(y * height_ratio)
    output[x, y] = input[src_x, src_y]
```

**Bilinear**:
```
for each output pixel (x, y):
    src_x, src_y = fractional source coordinates
    x1, y1 = floor(src_x, src_y)
    x2, y2 = x1+1, y1+1
    dx, dy = fractional parts
    
    # Weighted average of 4 nearest pixels
    value = p11 * (1-dx) * (1-dy) +
            p21 * dx * (1-dy) +
            p12 * (1-dx) * dy +
            p22 * dx * dy
```

### Crop Algorithm

Direct memory copy of rectangular regions, optimized with `copy_from_slice` for row-wise copying.

### Rotate Algorithm

**Fixed Angle (90°/180°/270°)**:

Pixel remapping based on rotation angle:
- **90° CW**: `(x, y) → (height-1-y, x)`
- **180°**: `(x, y) → (width-1-x, height-1-y)`
- **270° CW**: `(x, y) → (y, width-1-x)`

**Custom Angle (Arbitrary)**:

Uses inverse rotation with interpolation:

1. Calculate output dimensions to fit entire rotated image
2. For each output pixel:
   - Apply inverse rotation transformation
   - Sample source image at calculated position
   - Use interpolation for sub-pixel accuracy

```
For angle θ in radians:
cos_θ = cos(θ)
sin_θ = sin(θ)

# Inverse rotation from output to source coordinates
src_x = (dst_x - center_x) * cos_θ + (dst_y - center_y) * sin_θ + center_x
src_y = -(dst_x - center_x) * sin_θ + (dst_y - center_y) * cos_θ + center_y
```

## Future Enhancements

Potential additions to the transform module:

- [ ] Additional interpolation methods (bicubic, Lanczos)
- [ ] Arbitrary angle rotation (with interpolation)
- [ ] Flip operations (horizontal/vertical)
- [ ] Perspective transforms
- [ ] Affine transformations
- [ ] SIMD optimizations
- [ ] GPU acceleration support

## API Reference

For complete API documentation, run:

```bash
cargo doc --open
```

Then navigate to the `transform` module documentation.