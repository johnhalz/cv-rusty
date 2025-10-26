# Window Display Feature - Implementation Summary

## Overview

Added image display functionality to the cv-rusty library. This feature allows users to display images in GUI windows for debugging and visualization purposes. The API uses a unified `show_image()` function that works with both grayscale and color images through Rust's trait system.

## Changes Made

### 1. Dependencies (Cargo.toml)

- Added `minifb` v0.27 as an optional dependency
- Created new `window` feature flag that depends on `std` and `minifb`

### 2. New Module (src/window.rs)

Created a comprehensive window display module with the following functions:

#### Public API

- **`show_image(window_name, image)`** - Display any image (works with both Matrix1 and Matrix3)
- **`show_and_wait(window_name, image)`** - Display any image and wait for user to close
- **`wait_key(delay)`** - Wait for specified milliseconds (simplified version)
- **`Displayable` trait** - Trait for types that can be displayed in a window
- **`WindowError`** - Error type for window operations

#### Error Handling

- `WindowError` enum with variants:

  - `WindowCreation(String)` - Window creation/update failures
  - `InvalidDimensions` - Zero width or height images

#### Features

- Cross-platform support (Windows, macOS, Linux)
- Automatic RGB format conversion for Matrix3
- Grayscale to RGB conversion for Matrix1
- 60 FPS frame limit
- ESC key and window close button support

### 3. Library Integration (src/lib.rs)

- Added `window` module with feature gate
- Exported all public window functions when `window` feature is enabled

### 4. Examples

Created two comprehensive examples:

#### simple_show_image.rs

- Basic usage demonstration
- Creates a simple test pattern with red square and blue border
- Shows minimal code required to display an image

#### window_display_example.rs

- Comprehensive demonstration of all features
- Color gradient generation
- Grayscale radial gradient
- Checkerboard pattern
- Optional file loading if test.jpg exists

### 5. Documentation

#### docs/window_display.md
Complete API documentation including:

- Feature requirements
- API reference for all functions
- Complete examples
- Error handling guide
- Comparison with OpenCV
- Limitations and advanced usage
- Troubleshooting guide

#### README.md Updates

- Added `window` feature to Feature Flags section
- Added "Displaying Images in Windows" usage section with example
- Included Cargo.toml configuration instructions

#### examples/README.md Updates

- Added "Window Display (GUI)" section
- Documented both examples with usage instructions
- Added feature flags section
- Updated troubleshooting with window-specific issues

## Usage

### Enable the Feature

Add to your `Cargo.toml`:

```toml
[dependencies]
cv-rusty = { version = "0.3.0", features = ["window"] }
```

### Basic Example

```rust
use cv_rusty::{Matrix3, Matrix1, show_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Works with color images
    let mut color_image = Matrix3::zeros(400, 300);

    // Draw something
    for y in 100..200 {
        for x in 150..250 {
            color_image.set_pixel(x, y, 255, 0, 0);
        }
    }

    // Display the color image
    show_image("My Window", &color_image)?;

    // Also works with grayscale images
    let gray_image = Matrix1::zeros(400, 300);
    show_image("Grayscale", &gray_image)?;

    Ok(())
}
```

### Run Examples

```bash
# Simple example
cargo run --example simple_show_image --features window

# Comprehensive example
cargo run --example window_display_example --features window
```

## Technical Details

### Image Format Conversion

- **Matrix3 (RGB)**: Converts `[R, G, B, R, G, B, ...]` to minifb's `0x00RRGGBB` format
- **Matrix1 (Grayscale)**: Converts single channel to RGB by duplicating value: `gray -> (gray, gray, gray)`

### Window Behavior

- Each window blocks execution until closed
- Windows run at maximum 60 FPS
- ESC key or window close button exits the display
- Sequential display model (not concurrent like OpenCV)

### Dependencies

- `minifb` v0.27: Lightweight cross-platform windowing library
- Requires GUI support (not available in headless environments)

## Design Decisions

### Why minifb?

- Lightweight and cross-platform
- Simple API that matches well with our use case
- No complex dependencies
- Works directly with RGB buffers

### Optional Feature

Made it an optional feature because:

- Maintains `no_std` compatibility for core library
- Doesn't add GUI dependencies for embedded/server use cases
- Users can opt-in only when needed

### API Design

Simple, intuitive API:

- Unified function name (`show_image`) works with all image types
- Uses Rust's trait system (`Displayable` trait) for type safety and extensibility
- Blocking API for easy usage
- Window name as first parameter
- Error handling with Result types (more Rust-idiomatic than OpenCV)

## Limitations

1. **Sequential Windows**: Each `show_image` call blocks until window is closed
2. **Simplified wait_key**: Just sleeps, doesn't return key codes
3. **No Window Management**: Cannot programmatically resize, move, or destroy windows
4. **Requires GUI**: Not usable in headless environments

## Future Enhancements (Potential)

- Non-blocking window display with concurrent windows
- Full keyboard event handling in wait_key
- Mouse event callbacks
- Window resize handling
- Zoom and pan controls
- Pixel value display on hover
- Save displayed image functionality

## Testing

- Compiles successfully with `--features window`
- Compiles successfully without window feature (no dependency added)
- All existing tests pass
- Examples compile and run correctly
- No breaking changes to existing API

## Compatibility

- Requires Rust 2021 edition
- Works on Windows, macOS, and Linux
- GUI support required at runtime
- Compatible with all existing cv-rusty features
