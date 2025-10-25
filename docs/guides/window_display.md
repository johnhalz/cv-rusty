# Window Display Module

The window display module provides functionality for displaying images in GUI windows. The `show_image()` function works with both grayscale and color images through Rust's trait system.

## Features

- Display grayscale images (`Matrix1`)
- Display color images (`Matrix3`)
- Simple API similar to OpenCV
- Cross-platform support (Windows, macOS, Linux)
- Automatic window management

## Requirements

This feature requires the `window` feature flag to be enabled:

```toml
[dependencies]
cv-rusty = { version = "0.3.0", features = ["window"] }
```

**Note**: The window feature requires GUI support and is not available in headless environments (e.g., CI servers, Docker containers without X11).

## API Overview

### `show_image(window_name, image)`

Displays an image in a window. Works with both grayscale (`Matrix1`) and color (`Matrix3`) images.

**Arguments:**
- `window_name: &str` - Name of the window
- `image: &T` - Image to display (any type implementing `Displayable` trait)

**Returns:** `Result<(), WindowError>`

**Examples:**
```rust
use cv_rusty::{Matrix1, Matrix3, show_image};

// Display a color image
let color_image = Matrix3::zeros(640, 480);
show_image("Color Window", &color_image)?;

// Display a grayscale image
let gray_image = Matrix1::zeros(640, 480);
show_image("Grayscale Window", &gray_image)?;
```

### `show_and_wait(window_name, image)`

Displays an image and waits for user to close the window. Works with both color and grayscale images.

**Arguments:**
- `window_name: &str` - Name of the window
- `image: &T` - Image to display (any type implementing `Displayable` trait)

**Returns:** `Result<(), WindowError>`

**Examples:**
```rust
use cv_rusty::{Matrix1, Matrix3, show_and_wait};

// Works with color images
let color_image = Matrix3::zeros(640, 480);
show_and_wait("Color Window", &color_image)?;

// Works with grayscale images
let gray_image = Matrix1::zeros(640, 480);
show_and_wait("Grayscale Window", &gray_image)?;
```

### `wait_key(delay)`

Waits for a key press for a specified duration (simplified version).

**Arguments:**
- `delay: u64` - The number of milliseconds to wait. Use 0 to wait indefinitely.

**Example:**
```rust
use cv_rusty::wait_key;

wait_key(1000); // Wait for 1 second
```

**Note**: This is a simplified implementation that just sleeps. For more complex event handling with multiple windows, consider using the minifb window handle directly.

## Complete Example

```rust
use cv_rusty::{read_jpeg, show_image, Matrix3, Matrix1};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load an image from disk
    let image = read_jpeg("input.jpg")?;
    println!("Loaded {}x{} image", image.width(), image.height());
    
    // Display the image
    show_image("Original Image", &image)?;
    
    // Create a modified version
    let mut modified = image.clone();
    for y in 100..200 {
        for x in 150..250 {
            modified.set_pixel(x, y, 255, 0, 0);
        }
    }
    
    // Display the modified image
    show_image("Modified Image", &modified)?;
    
    // Convert to grayscale and display
    let gray = image.to_grayscale(cv_rusty::GrayscaleMethod::Average);
    show_image("Grayscale", &gray)?;
    
    Ok(())
}
```

## Keyboard Controls

- **ESC** - Close the window
- **Window close button** - Close the window

## Window Behavior

Each call to `show_image` creates a new window that:
- Opens immediately with the specified image
- Runs at a maximum of 60 FPS
- Remains open until the user presses ESC or closes the window
- Blocks execution until closed

## Error Handling

The window functions return `Result<(), WindowError>` with the following error types:

- `WindowError::WindowCreation(String)` - Failed to create or update the window
- `WindowError::InvalidDimensions` - Image has zero width or height
## Error Handling

**Example:**
```rust
use cv_rusty::{Matrix3, show_image, WindowError};

let image = Matrix3::zeros(640, 480);

match show_image("My Window", &image) {
    Ok(_) => println!("Image displayed successfully"),
    Err(WindowError::InvalidDimensions) => {
        eprintln!("Image has invalid dimensions");
    }
    Err(WindowError::WindowCreation(msg)) => {
        eprintln!("Failed to create window: {}", msg);
    }
}
```

## Comparison with OpenCV

| Feature | cv-rusty | OpenCV |
|---------|----------|--------|
| Display any image | `show_image("name", &image)` | `imshow("name", image)` |
| Wait for key | `wait_key(delay)` (simplified) | `waitKey(delay)` |
| Close window | ESC or close button | `destroyWindow()` or ESC |
| Multiple windows | Sequential | Concurrent |
| Type system | Unified API via traits | Single function for all types |

## Limitations

1. **Sequential windows**: Unlike OpenCV, windows are displayed sequentially. Each `show_image` call blocks until the window is closed.
2. **Simplified wait_key**: The `wait_key` function is a simplified sleep implementation and doesn't return key codes.
3. **No window management**: Windows cannot be moved, resized programmatically, or destroyed without user interaction.
4. **Requires GUI**: Cannot be used in headless environments.

## Advanced Usage

### Creating Custom Displayable Types

You can implement the `Displayable` trait for your own image types:

```rust
use cv_rusty::{Displayable, WindowError, show_image};

struct MyImage {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Displayable for MyImage {
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError> {
        if self.width == 0 || self.height == 0 {
            return Err(WindowError::InvalidDimensions);
        }
        
        let buffer: Vec<u32> = self.data
            .iter()
            .map(|&pixel| {
                let rgb = pixel as u32;
                (rgb << 16) | (rgb << 8) | rgb
            })
            .collect();
        
        Ok((buffer, self.width, self.height))
    }
}

// Now you can use show_image with your custom type
let my_image = MyImage { data: vec![128; 640 * 480], width: 640, height: 480 };
show_image("Custom Image", &my_image)?;
```

For more advanced window control, you can use the underlying `minifb` crate directly by converting images using the `Displayable` trait.

## Examples

See the following examples for more demonstrations:

- `examples/simple_show_image.rs` - Basic usage
- `examples/window_display_example.rs` - Comprehensive examples with gradients and patterns

Run examples with:
```bash
cargo run --example simple_show_image --features window
cargo run --example window_display_example --features window
```

## Troubleshooting

### "This example requires the 'window' feature"
Enable the window feature: `cargo run --example simple_show_image --features window`

### "Failed to create window"
- Ensure you have GUI support (not running in headless environment)
- On Linux, ensure X11 or Wayland is available
- Check that image dimensions are valid (> 0)

### Window doesn't appear
- Check that the image has valid dimensions
- Ensure the program isn't terminating immediately after the show_image call
- Try adding error handling to see if an error is being silently ignored

### Image colors look wrong
- Ensure your image data is in RGB format (not BGR)

## Benefits of the Unified API

The unified `show_image()` function provides several advantages:

1. **Single Function**: No need to remember separate functions for color vs grayscale
2. **Type Safe**: The compiler ensures you're passing a displayable image type
3. **Extensible**: You can implement `Displayable` for custom image types
4. **Clean Code**: More concise and easier to read

**Before (separate functions):**
```rust
show_image_color("Color", &color_image)?;
show_image("Grayscale", &gray_image)?;
```

**After (unified function):**
```rust
show_image("Color", &color_image)?;
show_image("Grayscale", &gray_image)?;
```
- cv-rusty uses RGB ordering: `[R, G, B, R, G, B, ...]`
