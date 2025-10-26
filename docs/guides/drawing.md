# Drawing Shapes Guide

The drawing module provides functionality to render shapes (rectangles and circles) on both grayscale (`Matrix1`) and RGB (`Matrix3`) images.

## Table of Contents

- [Overview](#overview)
- [Color System](#color-system)
- [Drawing Rectangles](#drawing-rectangles)
- [Drawing Circles](#drawing-circles)
- [Performance Considerations](#performance-considerations)
- [Examples](#examples)
- [Advanced Techniques](#advanced-techniques)

## Overview

The drawing module is `no_std` compatible and provides:

- ✅ **Rectangle drawing** with rotation, stroke, and fill
- ✅ **Circle drawing** with stroke and fill
- ✅ **Grayscale support** via `Matrix1`
- ✅ **RGB support** via `Matrix3`
- ✅ **Flexible styling** with customizable colors, stroke width, and fill
- ✅ **Rotation support** for rectangles (in degrees)
- ✅ **Opacity/transparency** support with alpha blending
- ✅ **Anti-aliasing-free** rendering (pixel-perfect)

## Color System

The `Color` enum provides a unified way to specify colors for both grayscale and RGB images:

```rust
use cv_rusty::drawing::Color;

// Create colors using constructors
let black = Color::black();           // RGB(0, 0, 0)
let white = Color::white();           // RGB(255, 255, 255)
let red = Color::rgb(255, 0, 0);      // RGB color
let gray = Color::gray(128);          // Grayscale

// Create colors from hex strings
let blue = Color::from_hex("#0000FF").unwrap();      // 6-digit with #
let green = Color::from_hex("00FF00").unwrap();      // 6-digit without #
let cyan = Color::from_hex("#0FF").unwrap();         // 3-digit format
let magenta = Color::from_hex("F0F").unwrap();       // 3-digit without #

// Parse from strings using FromStr trait
let yellow: Color = "#FFFF00".parse().unwrap();
let orange: Color = "FF8800".parse().unwrap();

// Create colors with custom opacity (0.0 = fully transparent, 1.0 = fully opaque)
let semi_red = Color::rgb_with_opacity(255, 0, 0, 0.5);      // 50% transparent red
let semi_gray = Color::gray_with_opacity(128, 0.7);          // 70% opaque gray

// Modify opacity of existing colors
let opaque_blue = Color::rgb(0, 0, 255);
let transparent_blue = opaque_blue.with_opacity(0.3);        // 30% opaque blue

// Get opacity value
let opacity = semi_red.opacity();     // 0.5

// Conversions
let rgb_tuple = red.to_rgb();         // (255, 0, 0)
let gray_value = red.to_gray();       // 76 (using luminance formula)
```

### Hex Color Format

The `Color::from_hex()` method and `FromStr` trait support multiple hex formats:

**6-digit format (RRGGBB):**
- `"#FF0000"` or `"FF0000"` → Red (255, 0, 0)
- `"#00FF00"` or `"00FF00"` → Green (0, 255, 0)
- `"#0000FF"` or `"0000FF"` → Blue (0, 0, 255)

**3-digit format (RGB):**
- `"#F00"` or `"F00"` → Red (255, 0, 0) - expands to FF0000
- `"#0F0"` or `"0F0"` → Green (0, 255, 0) - expands to 00FF00
- `"#00F"` or `"00F"` → Blue (0, 0, 255) - expands to 0000FF

The 3-digit format expands each digit: `F` → `FF`, `0` → `00`, etc.
This means `#369` expands to `#336699` (RGB: 51, 102, 153).

**Case insensitive:**
- `"#FFFFFF"`, `"#ffffff"`, or `"#FfFfFf"` all work

**With or without hash:**
- Both `"#FF0000"` and `"FF0000"` are valid

### Opacity and Alpha Blending

Colors support opacity values between 0.0 (fully transparent) and 1.0 (fully opaque). When drawing with semi-transparent colors, the new color is blended with the existing pixel using alpha blending:

```
result = existing_color * (1 - opacity) + new_color * opacity
```

**Key features:**
- Default opacity is `1.0` (fully opaque) for all standard constructors
- Opacity is automatically clamped to the range `[0.0, 1.0]`
- Works with both grayscale and RGB images
- Fully transparent colors (opacity = 0.0) do not modify pixels
- Blending occurs for each color channel independently

```rust
// Create semi-transparent colors
let semi_red = Color::rgb_with_opacity(255, 0, 0, 0.5);
let semi_gray = Color::gray_with_opacity(200, 0.3);

// Modify opacity of existing colors
let blue = Color::rgb(0, 0, 255);
let transparent_blue = blue.with_opacity(0.4);

// Draw with transparency
draw_circle(
    &mut image,
    320.0, 240.0,
    50.0,
    None,
    Some(Color::rgb_with_opacity(255, 0, 0, 0.6))  // 60% opaque red
);
```

### Grayscale Conversion

When drawing RGB colors on grayscale images or vice versa, automatic conversion occurs:

- **RGB → Gray**: Uses standard luminance formula: `0.299*R + 0.587*G + 0.114*B`
- **Gray → RGB**: Replicates gray value across all channels: `(gray, gray, gray)`

## Drawing Rectangles

Rectangles are drawn with their center at the specified `(x, y)` position.

### Function Signatures

```rust
// Works with both RGB (Matrix3) and grayscale (Matrix1) images
pub fn draw_rectangle<T: DrawTarget>(
    image: &mut T,
    x: f32,                    // X coordinate of center
    y: f32,                    // Y coordinate of center
    width: f32,                // Width of rectangle
    height: f32,               // Height of rectangle
    rotation: f32,             // Rotation in degrees (clockwise)
    stroke: Option<Stroke>,    // Optional stroke with width and color
    fill_color: Option<Color>, // Fill color (None for no fill)
)
```

### Basic Rectangle

```rust
use cv_rusty::{Matrix3, Matrix1, draw_rectangle, Color, Stroke};

// Works with RGB images
let mut rgb_image = Matrix3::zeros(480, 640);
draw_rectangle(
    &mut rgb_image,
    320.0, 240.0,  // Center at (320, 240)
    100.0, 60.0,   // 100 pixels wide, 60 pixels tall
    0.0,           // No rotation
    Some(Stroke::new(2, Color::rgb(0, 0, 0))),  // 2px black border
    Some(Color::rgb(255, 0, 0))                 // Red fill
);

// Also works with grayscale images
let mut gray_image = Matrix1::zeros(480, 640);
draw_rectangle(
    &mut gray_image,
    320.0, 240.0,
    100.0, 60.0,
    0.0,
    Some(Stroke::new(2, Color::gray(255))),  // 2px white border
    Some(Color::gray(100))                   // Dark gray fill
);
```

### Rotated Rectangle

```rust
// Draw a green rectangle rotated 45 degrees (works with any image type)
draw_rectangle(
    &mut rgb_image,
    200.0, 150.0,  // Center position
    80.0, 120.0,   // Width and height
    45.0,          // 45 degrees clockwise
    Some(Stroke::new(3, Color::rgb(255, 255, 255))),  // 3px white border
    Some(Color::rgb(0, 255, 0))                       // Green fill
);
```

### Outline-Only Rectangle

```rust
// Draw just the outline (no fill, works with any image type)
draw_rectangle(
    &mut rgb_image,
    400.0, 300.0,
    150.0, 100.0,
    30.0,          // Rotated 30 degrees
    Some(Stroke::new(4, Color::rgb(0, 0, 255))),  // 4px blue outline
    None           // No fill
);
```

### Fill-Only Rectangle

```rust
// Draw filled rectangle without border (works with any image type)
draw_rectangle(
    &mut rgb_image,
    500.0, 200.0,
    60.0, 60.0,
    0.0,
    None,          // No stroke
    Some(Color::rgb(255, 255, 0))    // Yellow fill
);
```

### Semi-Transparent Rectangle

```rust
// Draw overlapping semi-transparent rectangles
draw_rectangle(
    &mut rgb_image,
    200.0, 200.0,
    150.0, 100.0,
    0.0,
    None,
    Some(Color::rgb(0, 0, 255))  // Opaque blue background
);

// Overlapping semi-transparent red rectangle
draw_rectangle(
    &mut rgb_image,
    250.0, 220.0,
    150.0, 100.0,
    0.0,
    None,
    Some(Color::rgb_with_opacity(255, 0, 0, 0.5))  // 50% transparent red
);
// The overlap will show a purple blend
```

## Drawing Circles

Circles are drawn with their center at the specified `(x, y)` position.

### Function Signatures

```rust
// Works with both RGB (Matrix3) and grayscale (Matrix1) images
pub fn draw_circle<T: DrawTarget>(
    image: &mut T,
    x: f32,                    // X coordinate of center
    y: f32,                    // Y coordinate of center
    radius: f32,               // Radius of circle
    stroke: Option<Stroke>,    // Optional stroke with width and color
    fill_color: Option<Color>, // Fill color (None for no fill)
)
```

### Basic Circle

```rust
use cv_rusty::{Matrix3, Matrix1, draw_circle, Color, Stroke};

// Works with RGB images
let mut rgb_image = Matrix3::zeros(480, 640);
draw_circle(
    &mut rgb_image,
    320.0, 240.0,  // Center at (320, 240)
    50.0,          // Radius of 50 pixels
    Some(Stroke::new(3, Color::rgb(255, 255, 255))),  // 3px white border
    Some(Color::rgb(0, 0, 255))                       // Blue fill
);

// Also works with grayscale images
let mut gray_image = Matrix1::zeros(480, 640);
draw_circle(
    &mut gray_image,
    320.0, 240.0,
    50.0,
    Some(Stroke::new(3, Color::gray(255))),  // 3px white border
    Some(Color::gray(100))                   // Dark gray fill
);
```

### Outline-Only Circle

```rust
// Draw just the outline (no fill, works with any image type)
draw_circle(
    &mut rgb_image,
    200.0, 200.0,
    60.0,          // Radius
    Some(Stroke::new(5, Color::rgb(255, 0, 0))),  // 5px red outline
    None           // No fill
);
```

### Fill-Only Circle

```rust
// Draw filled circle without border (works with any image type)
draw_circle(
    &mut rgb_image,
    450.0, 350.0,
    40.0,          // Radius
    None,          // No stroke
    Some(Color::rgb(0, 255, 0))      // Green fill
);
```

### Semi-Transparent Circle

```rust
// Draw overlapping semi-transparent circles for a Venn diagram effect
draw_circle(
    &mut rgb_image,
    200.0, 240.0,
    60.0,
    Some(Stroke::new(2, Color::rgb(200, 0, 0))),
    Some(Color::rgb_with_opacity(255, 0, 0, 0.6))  // 60% opaque red
);

draw_circle(
    &mut rgb_image,
    260.0, 240.0,
    60.0,
    Some(Stroke::new(2, Color::rgb(0, 200, 0))),
    Some(Color::rgb_with_opacity(0, 255, 0, 0.6))  // 60% opaque green
);

draw_circle(
    &mut rgb_image,
    230.0, 290.0,
    60.0,
    Some(Stroke::new(2, Color::rgb(0, 0, 200))),
    Some(Color::rgb_with_opacity(0, 0, 255, 0.6))  // 60% opaque blue
);
// Overlapping areas blend the colors
```

## Unified API for All Image Types

The drawing functions work seamlessly with both RGB (`Matrix3`) and grayscale (`Matrix1`) images using the `DrawTarget` trait:

```rust
use cv_rusty::{Matrix1, Matrix3, draw_rectangle, draw_circle, Color};

// Create both image types
let mut rgb_image = Matrix3::zeros(480, 640);
let mut gray_image = Matrix1::zeros(480, 640);

// Same function works for both!
draw_rectangle(
    &mut rgb_image,
    200.0, 150.0,
    100.0, 80.0,
    15.0,
    2,
    Some(Color::rgb(255, 255, 255)),  // White border
    Some(Color::rgb(255, 0, 0))       // Red fill
);

draw_rectangle(
    &mut gray_image,
    200.0, 150.0,
    100.0, 80.0,
    15.0,
    2,
    Some(Color::gray(255)),  // White border
    Some(Color::gray(50))    // Dark gray fill
);

// Colors are automatically converted to the appropriate format
draw_circle(
    &mut gray_image,
    400.0, 300.0,
    60.0,
    3,
    Some(Color::rgb(0, 0, 0)),    // RGB black automatically converts to gray
    Some(Color::gray(200))        // Light gray fill
);
```

## Performance Considerations

### Rectangle Drawing

- **Axis-aligned rectangles** (rotation = 0°) are slightly faster than rotated ones
- **Rotation** uses scanline rendering with point-in-polygon tests
- **Time complexity**: O(bounding_box_area) for rotated rectangles
- **Best for**: UI elements, bounding boxes, region marking

### Circle Drawing

- Uses efficient **circle filling algorithm** with distance checks
- **Time complexity**: O(bounding_box_area)
- Stroke width is centered on the circle's radius
- **Best for**: Markers, highlights, buttons, pie charts

### Optimization Tips

1. **Batch drawing operations** when possible
2. **Avoid overdraw** of large filled shapes when possible
3. **Use appropriate stroke widths** - very large stroke widths can be slow
4. **Consider clipping** to image bounds before drawing off-screen shapes

### Typical Performance

On a modern CPU (example: Apple M1):

| Operation | Image Size | Time | Notes |
|-----------|------------|------|-------|
| Circle (r=50) | 640×480 | ~50 µs | With fill and stroke |
| Rectangle | 640×480 | ~30 µs | Axis-aligned |
| Rectangle (rotated) | 640×480 | ~45 µs | 45° rotation |
| 100 shapes | 640×480 | ~4 ms | Mixed circles and rectangles |

## Examples

### Complete Drawing Example

```rust
use cv_rusty::{Matrix3, draw_rectangle, draw_circle, write_png, Color, Stroke};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create white canvas
    let mut image = Matrix3::zeros(600, 800);
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }

    // Draw background elements
    draw_rectangle(
        &mut image,
        400.0, 300.0,
        700.0, 500.0,
        0.0,
        None,
        Some(Color::rgb(240, 240, 255))  // Light blue background
    );

    // Draw a title bar
    draw_rectangle(
        &mut image,
        400.0, 50.0,
        700.0, 60.0,
        0.0,
        Some(Stroke::new(2, Color::rgb(100, 100, 100))),
        Some(Color::rgb(60, 120, 200))
    );

    // Draw some UI elements
    for i in 0..5 {
        let x = 150.0 + (i as f32 * 130.0);
        draw_circle(
            &mut image,
            x, 150.0,
            40.0,
            Some(Stroke::new(3, Color::rgb(50, 50, 50))),
            Some(Color::rgb(100, 200, 100))
        );
    }

    // Draw rotated indicator
    draw_rectangle(
        &mut image,
        400.0, 450.0,
        200.0, 30.0,
        -10.0,  // Slightly rotated
        Some(Stroke::new(2, Color::rgb(0, 0, 0))),
        Some(Color::rgb(255, 200, 50))
    );

    write_png(&image, "ui_mockup.png")?;
    Ok(())
}
```

### Data Visualization

```rust
use cv_rusty::{Matrix3, draw_circle, draw_rectangle, Color, Stroke};

fn draw_bar_chart(data: &[f32]) -> Matrix3 {
    let mut image = Matrix3::zeros(400, 600);
    
    // White background
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.set_pixel(x, y, 255, 255, 255);
        }
    }
    
    let bar_width = 40.0;
    let spacing = 20.0;
    let max_height = 300.0;
    let base_y = 350.0;
    
    for (i, &value) in data.iter().enumerate() {
        let x = 50.0 + (i as f32 * (bar_width + spacing));
        let height = value * max_height;
        
        draw_rectangle(
            &mut image,
            x + bar_width / 2.0,
            base_y - height / 2.0,
            bar_width,
            height,
            0.0,
            Some(Stroke::new(2, Color::rgb(0, 0, 0))),
            Some(Color::rgb(100, 150, 255))
        );
        
        // Data point marker
        draw_circle(
            &mut image,
            x + bar_width / 2.0,
            base_y - height,
            5.0,
            Some(Stroke::new(1, Color::rgb(0, 0, 0))),
            Some(Color::rgb(255, 100, 100))
        );
    }
    
    image
}

// Usage
let data = vec![0.3, 0.7, 0.5, 0.9, 0.4, 0.6, 0.8];
let chart = draw_bar_chart(&data);
```

### Annotation Tool

```rust
use cv_rusty::{Matrix3, draw_rectangle, draw_circle, Color, Stroke};

fn annotate_image(
    image: &mut Matrix3,
    bbox: (f32, f32, f32, f32),  // x, y, width, height
    confidence: f32
) {
    let (x, y, w, h) = bbox;
    
    // Draw bounding box
    let color = if confidence > 0.8 {
        Color::rgb(0, 255, 0)  // Green for high confidence
    } else if confidence > 0.5 {
        Color::rgb(255, 255, 0)  // Yellow for medium
    } else {
        Color::rgb(255, 0, 0)  // Red for low confidence
    };
    
    draw_rectangle(
        image,
        x + w / 2.0,
        y + h / 2.0,
        w, h,
        0.0,
        Some(Stroke::new(3, color)),
        None  // No fill, just outline
    );
    
    // Draw confidence indicator (corner circles)
    let radius = 5.0;
    for (cx, cy) in [
        (x, y), (x + w, y), (x + w, y + h), (x, y + h)
    ] {
        draw_circle(
            image,
            cx, cy,
            radius,
            None,
            Some(color)
        );
    }
}
```

## Advanced Techniques

### Layered Drawing

Draw shapes in order from back to front for proper layering:

```rust
// 1. Draw background
draw_rectangle(&mut image, 400.0, 300.0, 700.0, 500.0, 0.0, 0, None, 
               Some(Color::rgb(200, 200, 200)));

// 2. Draw middle layer
draw_circle(&mut image, 400.0, 300.0, 100.0, 0, None,
            Some(Color::rgb(100, 100, 255)));

// 3. Draw foreground
draw_rectangle(&mut image, 400.0, 300.0, 50.0, 50.0, 45.0, 2,
               Some(Color::rgb(0, 0, 0)), Some(Color::rgb(255, 255, 255)));
```

### Shape Combinations

Create complex shapes by combining primitives:

```rust
// Draw a target/bullseye
let center_x = 400.0;
let center_y = 300.0;

for i in 0..5 {
    let radius = 100.0 - (i as f32 * 20.0);
    let color = if i % 2 == 0 {
        Color::rgb(255, 0, 0)
    } else {
        Color::rgb(255, 255, 255)
    };
    
    draw_circle(&mut image, center_x, center_y, radius, 0, None, Some(color));
}
```

### Grid Patterns

```rust
// Draw a grid
let cols = 10;
let rows = 8;
let cell_size = 50.0;
let start_x = 100.0;
let start_y = 50.0;

for row in 0..rows {
    for col in 0..cols {
        let x = start_x + (col as f32 * cell_size);
        let y = start_y + (row as f32 * cell_size);
        
        let fill = if (row + col) % 2 == 0 {
            Color::rgb(200, 200, 200)
        } else {
            Color::rgb(255, 255, 255)
        };
        
        draw_rectangle(
            &mut image,
            x + cell_size / 2.0,
            y + cell_size / 2.0,
            cell_size,
            cell_size,
            0.0,
            1,
            Some(Color::rgb(100, 100, 100)),
            Some(fill)
        );
    }
}
```

## Best Practices

1. **Pre-calculate coordinates** when drawing many shapes
2. **Use appropriate data types** - functions accept `f32` for smooth positioning
3. **Handle edge cases** - shapes are clipped to image bounds automatically
4. **Layer thoughtfully** - later draws overwrite earlier ones
5. **Consider color contrast** - ensure shapes are visible against background
6. **Test with various rotations** - especially for rectangles
7. **Validate inputs** - ensure dimensions and positions are reasonable

## Limitations

- No anti-aliasing (pixel-perfect rendering only)
- No line drawing (use thin rectangles as workaround)
- No ellipse support (use circles only)
- No polygon support (use multiple rectangles)
- Rectangle rotation is centered (not arbitrary pivot points)
- Transparency blending is simple alpha compositing (no premultiplied alpha)

## Future Enhancements

Potential additions in future versions:

- Line drawing with thickness
- Polygon drawing
- Ellipse support
- Anti-aliasing options
- Premultiplied alpha blending
- Gradient fills
- Pattern fills
- Bezier curves
- Custom pivot points for rotation

## See Also

- [Matrix Guide](./matrix.md) - Working with image matrices
- [Color Conversion Guide](./color.md) - Color space conversions
- [Transform Guide](./transform.md) - Image transformations
- [API Documentation](https://docs.rs/cv-rusty) - Full API reference