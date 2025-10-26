# Color Conversion Quick Reference Guide

This guide provides quick examples and reference information for using the color space conversion features in cv-rusty.

## Table of Contents

1. [Matrix Types](#matrix-types)
2. [RGB to Grayscale](#rgb-to-grayscale)
3. [RGB ↔ HSV Conversions](#rgb--hsv-conversions)
4. [RGB ↔ HSL Conversions](#rgb--hsl-conversions)
5. [Practical Examples](#practical-examples)
6. [Performance Tips](#performance-tips)

## Matrix Types

### Matrix3 - RGB Images

Three-channel matrix for RGB color images:

```rust
use cv_rusty::Matrix3;

// Create a 640x480 RGB image
let mut rgb_image = Matrix3::zeros(640, 480);

// Set a red pixel at (100, 50)
rgb_image.set_pixel(100, 50, 255, 0, 0);

// Get pixel value
if let Some((r, g, b)) = rgb_image.get_pixel(100, 50) {
    println!("RGB: ({}, {}, {})", r, g, b);
}
```

### Matrix1 - Grayscale Images

Single-channel matrix for grayscale images:

```rust
use cv_rusty::Matrix1;

// Create a 640x480 grayscale image
let mut gray_image = Matrix1::zeros(640, 480);

// Set a pixel value
gray_image.set_pixel(100, 50, 128);

// Get pixel value
if let Some(value) = gray_image.get_pixel(100, 50) {
    println!("Gray value: {}", value);
}
```

## RGB to Grayscale

### Three Conversion Methods

#### 1. Luminosity Method (Recommended)

Accounts for human perception - green appears brightest to our eyes.

```rust
use cv_rusty::Matrix3;

let rgb_image = Matrix3::zeros(640, 480);
let gray = rgb_image.to_grayscale();
```

**Formula**: `0.299*R + 0.587*G + 0.114*B`

**Best for**: General purpose, photography, human-viewed images

#### 2. Average Method

Simple arithmetic mean of RGB channels.

```rust
let gray = rgb_image.to_grayscale_average();
```

**Formula**: `(R + G + B) / 3`

**Best for**: Quick conversions, when perceptual accuracy isn't critical

#### 3. Lightness Method

Midpoint between the maximum and minimum RGB values.

```rust
let gray = rgb_image.to_grayscale_lightness();
```

**Formula**: `(max(R,G,B) + min(R,G,B)) / 2`

**Best for**: Preserving color range information

### Using Method Parameter

```rust
use cv_rusty::{Matrix3, GrayscaleMethod};

let rgb_image = Matrix3::zeros(640, 480);

// Choose method explicitly
let gray = rgb_image.to_grayscale_with_method(GrayscaleMethod::Luminosity);
let gray = rgb_image.to_grayscale_with_method(GrayscaleMethod::Average);
let gray = rgb_image.to_grayscale_with_method(GrayscaleMethod::Lightness);
```

### Comparison of Methods

| Color | RGB | Luminosity | Average | Lightness |
|-------|-----|------------|---------|-----------|
| Red | (255, 0, 0) | 76 | 85 | 127 |
| Green | (0, 255, 0) | 149 | 85 | 127 |
| Blue | (0, 0, 255) | 29 | 85 | 127 |
| White | (255, 255, 255) | 255 | 255 | 255 |

## RGB ↔ HSV Conversions

HSV (Hue, Saturation, Value) is useful for color-based segmentation and manipulation.

### RGB to HSV

```rust
use cv_rusty::rgb_to_hsv;

let (h, s, v) = rgb_to_hsv(255, 0, 0); // Red
// h = 0.0° (hue in degrees, 0-360)
// s = 1.0 (saturation, 0.0-1.0)
// v = 1.0 (value/brightness, 0.0-1.0)
```

### HSV to RGB

```rust
use cv_rusty::hsv_to_rgb;

let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0); // Red
// r = 255, g = 0, b = 0
```

### HSV Color Wheel

- **Hue (H)**: Color type

  - 0° = Red
  - 60° = Yellow
  - 120° = Green
  - 180° = Cyan
  - 240° = Blue
  - 300° = Magenta
  - 360° = Red (wraps around)

- **Saturation (S)**: Color intensity

  - 0.0 = Gray (no color)
  - 1.0 = Pure color

- **Value (V)**: Brightness

  - 0.0 = Black
  - 1.0 = Full brightness

### Common HSV Operations

#### Make color brighter/darker
```rust
let (h, s, v) = rgb_to_hsv(r, g, b);
let brighter = hsv_to_rgb(h, s, v * 1.5);  // 50% brighter
let darker = hsv_to_rgb(h, s, v * 0.5);    // 50% darker
```

#### Increase/decrease saturation
```rust
let (h, s, v) = rgb_to_hsv(r, g, b);
let vibrant = hsv_to_rgb(h, s * 1.5, v);   // More saturated
let muted = hsv_to_rgb(h, s * 0.5, v);     // Less saturated
```

#### Shift hue (change color)
```rust
let (h, s, v) = rgb_to_hsv(r, g, b);
let shifted = hsv_to_rgb((h + 180.0) % 360.0, s, v); // Opposite color
```

## RGB ↔ HSL Conversions

HSL (Hue, Saturation, Lightness) is useful for color manipulation and adjustments.

### RGB to HSL

```rust
use cv_rusty::rgb_to_hsl;

let (h, s, l) = rgb_to_hsl(255, 0, 0); // Red
// h = 0.0° (hue in degrees, 0-360)
// s = 1.0 (saturation, 0.0-1.0)
// l = 0.5 (lightness, 0.0-1.0)
```

### HSL to RGB

```rust
use cv_rusty::hsl_to_rgb;

let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5); // Red
// r = 255, g = 0, b = 0
```

### HSL vs HSV

- **HSL Lightness**: 0.0 = black, 0.5 = pure color, 1.0 = white
- **HSV Value**: 0.0 = black, 1.0 = pure color (never white at full saturation)

**Use HSL when**: You need symmetric lightness control (pure color in middle)
**Use HSV when**: You need intuitive brightness control (brighter = higher value)

### Common HSL Operations

#### Lighten/darken color
```rust
let (h, s, l) = rgb_to_hsl(r, g, b);
let lighter = hsl_to_rgb(h, s, l * 1.2);   // 20% lighter
let darker = hsl_to_rgb(h, s, l * 0.8);    // 20% darker
```

#### Create tints and shades
```rust
let (h, s, l) = rgb_to_hsl(r, g, b);
let tint = hsl_to_rgb(h, s, (l + 1.0) / 2.0);  // Mix with white
let shade = hsl_to_rgb(h, s, l / 2.0);          // Mix with black
```

## Practical Examples

### Example 1: Color-Based Object Detection

```rust
use cv_rusty::{Matrix3, rgb_to_hsv};

fn detect_red_pixels(image: &Matrix3) -> Vec<(usize, usize)> {
    let mut red_pixels = Vec::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let (h, s, v) = rgb_to_hsv(r, g, b);

                // Red is at 0° and 360° (wraps around)
                if (h < 20.0 || h > 340.0) && s > 0.5 && v > 0.5 {
                    red_pixels.push((x, y));
                }
            }
        }
    }

    red_pixels
}
```

### Example 2: Automatic White Balance

```rust
use cv_rusty::{Matrix3, rgb_to_hsl, hsl_to_rgb};

fn auto_white_balance(image: &Matrix3) -> Matrix3 {
    let mut result = image.clone();

    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let (h, s, l) = rgb_to_hsl(r, g, b);
                // Reduce saturation for whites
                let adjusted_s = if l > 0.8 { s * 0.5 } else { s };
                let (new_r, new_g, new_b) = hsl_to_rgb(h, adjusted_s, l);
                result.set_pixel(x, y, new_r, new_g, new_b);
            }
        }
    }

    result
}
```

### Example 3: Contrast Enhancement

```rust
use cv_rusty::{Matrix3, rgb_to_hsv, hsv_to_rgb};

fn enhance_contrast(image: &Matrix3, factor: f32) -> Matrix3 {
    let mut result = image.clone();

    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let (h, s, v) = rgb_to_hsv(r, g, b);
                // Enhance value with gamma correction
                let enhanced_v = v.powf(1.0 / factor).min(1.0);
                let (new_r, new_g, new_b) = hsv_to_rgb(h, s, enhanced_v);
                result.set_pixel(x, y, new_r, new_g, new_b);
            }
        }
    }

    result
}
```

### Example 4: Edge Detection Preprocessing

```rust
use cv_rusty::Matrix3;

fn prepare_for_edge_detection(image: &Matrix3) -> Matrix1 {
    // Convert to grayscale using luminosity method
    // This is optimal for edge detection as it preserves
    // perceptual brightness differences
    image.to_grayscale()
}
```

### Example 5: Color Segmentation

```rust
use cv_rusty::{Matrix3, Matrix1, rgb_to_hsv};

fn segment_by_hue(image: &Matrix3, target_hue: f32, tolerance: f32) -> Matrix1 {
    let mut mask = Matrix1::zeros(image.width(), image.height());

    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                let (h, s, v) = rgb_to_hsv(r, g, b);

                // Check if hue is within tolerance
                let hue_diff = (h - target_hue).abs();
                let in_range = hue_diff < tolerance || hue_diff > (360.0 - tolerance);

                if in_range && s > 0.3 && v > 0.3 {
                    mask.set_pixel(x, y, 255);
                } else {
                    mask.set_pixel(x, y, 0);
                }
            }
        }
    }

    mask
}
```

## Performance Tips

### 1. Batch Conversions

When converting many pixels, iterate directly over the image data:

```rust
// Efficient: Direct iteration
for y in 0..image.height() {
    for x in 0..image.width() {
        if let Some((r, g, b)) = image.get_pixel(x, y) {
            let (h, s, v) = rgb_to_hsv(r, g, b);
            // Process...
        }
    }
}
```

### 2. Use Appropriate Grayscale Method

- **Luminosity**: Slower (uses floating point), most accurate
- **Average**: Fast (integer math), good approximation
- **Lightness**: Fastest (min/max only), least accurate

### 3. Avoid Repeated Conversions

```rust
// Bad: Converting back and forth
let (h, s, v) = rgb_to_hsv(r, g, b);
let (r2, g2, b2) = hsv_to_rgb(h, s, v);

// Good: Do all HSV operations together
let (h, s, v) = rgb_to_hsv(r, g, b);
let adjusted_v = v * 1.5;
let adjusted_s = s * 0.8;
let (r2, g2, b2) = hsv_to_rgb(h, adjusted_s, adjusted_v);
```

### 4. Memory Efficiency

Use `Matrix1` for grayscale images to save 2/3 memory:

```rust
// Matrix3: 640x480x3 = 921,600 bytes
let rgb = Matrix3::zeros(640, 480);

// Matrix1: 640x480x1 = 307,200 bytes
let gray = Matrix1::zeros(640, 480);
```

### 5. no_std Compatibility

All color conversion functions work in `no_std` environments:

```rust
#![no_std]

extern crate alloc;
use cv_rusty::{Matrix3, Matrix1, rgb_to_hsv};

// Works in embedded systems!
```

## Common Pitfalls

### 1. Hue Wraparound

Hue wraps around at 360°:

```rust
let h = 350.0;
let shifted = (h + 30.0) % 360.0; // = 20.0, not 380.0
```

### 2. Saturation/Value Clamping

Always clamp S and V to [0.0, 1.0]:

```rust
let s = (s * 1.5).min(1.0).max(0.0);
let v = (v * 1.5).min(1.0).max(0.0);
```

### 3. Roundtrip Precision

Expect minor differences due to rounding:

```rust
let original = (192, 64, 128);
let (h, s, v) = rgb_to_hsv(192, 64, 128);
let (r, g, b) = hsv_to_rgb(h, s, v);
// (r, g, b) might be (192, 64, 127) or (192, 63, 128)
// Difference is typically ±1 due to floating point conversion
```

### 4. Black/White Handling

Black and white have undefined hue:

```rust
let (h, s, v) = rgb_to_hsv(0, 0, 0);     // h is 0.0, but meaningless
let (h, s, v) = rgb_to_hsv(255, 255, 255); // h is 0.0, s is 0.0
```

## Further Reading

- [HSV Color Space - Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV)
- [Grayscale Conversion Algorithms](https://en.wikipedia.org/wiki/Grayscale)
- [Color Space Conversions](https://www.rapidtables.com/convert/color/)

## Support

For issues, questions, or contributions, please visit:
https://github.com/johnhalz/cv-rusty
