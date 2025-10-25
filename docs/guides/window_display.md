# Window Display Module

The window display module provides functionality similar to OpenCV's `imshow` and `waitKey` for displaying images in GUI windows.

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

### `imshow_color(window_name, image)`

Displays a color image (Matrix3) in a window.

**Arguments:**
- `window_name: &str` - The name of the window
- `image: &Matrix3` - The RGB image to display

**Returns:** `Result<(), WindowError>`

**Example:**
```rust
use cv_rusty::{Matrix3, imshow_color};

let image = Matrix3::zeros(640, 480);
imshow_color("My Window", &image)?;
```

### `imshow(window_name, image)`

Displays a grayscale image (Matrix1) in a window.

**Arguments:**
- `window_name: &str` - The name of the window
- `image: &Matrix1` - The grayscale image to display

**Returns:** `Result<(), WindowError>`

**Example:**
```rust
use cv_rusty::{Matrix1, imshow};

let image = Matrix1::zeros(640, 480);
imshow("Grayscale Window", &image)?;
```

### `show_and_wait(window_name, image)`

Convenience function that displays a color image and waits for the user to close it.

**Arguments:**
- `window_name: &str` - The name of the window
- `image: &Matrix3` - The RGB image to display

**Returns:** `Result<(), WindowError>`

**Example:**
```rust
use cv_rusty::{Matrix3, show_and_wait};

let image = Matrix3::zeros(640, 480);
show_and_wait("My Window", &image)?;
```

### `show_and_wait_gray(window_name, image)`

Convenience function that displays a grayscale image and waits for the user to close it.

**Arguments:**
- `window_name: &str` - The name of the window
- `image: &Matrix1` - The grayscale image to display

**Returns:** `Result<(), WindowError>`

**Example:**
```rust
use cv_rusty::{Matrix1, show_and_wait_gray};

let image = Matrix1::zeros(640, 480);
show_and_wait_gray("My Window", &image)?;
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
use cv_rusty::{read_jpeg, imshow_color, Matrix3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load an image from disk
    let image = read_jpeg("input.jpg")?;
    println!("Loaded {}x{} image", image.width(), image.height());
    
    // Display the image
    imshow_color("Original Image", &image)?;
    
    // Create a modified version
    let mut modified = image.clone();
    for y in 100..200 {
        for x in 100..200 {
            modified.set_pixel(x, y, 255, 0, 0); // Add red square
        }
    }
    
    // Display the modified image
    imshow_color("Modified Image", &modified)?;
    
    Ok(())
}
```

## Keyboard Controls

- **ESC** - Close the window
- **Window close button** - Close the window

## Window Behavior

Each call to `imshow` or `imshow_color` creates a new window that:
- Opens immediately with the specified image
- Runs at a maximum of 60 FPS
- Remains open until the user presses ESC or closes the window
- Blocks execution until closed

## Error Handling

The window functions return `Result<(), WindowError>` with the following error types:

- `WindowError::WindowCreation(String)` - Failed to create or update the window
- `WindowError::InvalidDimensions` - Image has zero width or height

**Example:**
```rust
use cv_rusty::{Matrix3, imshow_color, WindowError};

let image = Matrix3::zeros(640, 480);

match imshow_color("My Window", &image) {
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
| Display color image | `imshow_color("name", &image)` | `imshow("name", image)` |
| Display grayscale | `imshow("name", &image)` | `imshow("name", image)` |
| Wait for key | `wait_key(delay)` (simplified) | `waitKey(delay)` |
| Close window | ESC or close button | `destroyWindow()` or ESC |
| Multiple windows | Sequential | Concurrent |

## Limitations

1. **Sequential windows**: Unlike OpenCV, windows are displayed sequentially. Each `imshow` call blocks until the window is closed.
2. **Simplified wait_key**: The `wait_key` function is a simplified sleep implementation and doesn't return key codes.
3. **No window management**: Windows cannot be moved, resized programmatically, or destroyed without user interaction.
4. **Requires GUI**: Cannot be used in headless environments.

## Advanced Usage

For more advanced window control, you can use the underlying `minifb` crate directly:

```rust
use cv_rusty::Matrix3;
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = Matrix3::zeros(640, 480);
    
    // Create window with custom options
    let mut window = Window::new(
        "Custom Window",
        image.width(),
        image.height(),
        WindowOptions::default(),
    )?;
    
    // Convert image data to minifb format (0x00RRGGBB)
    let buffer: Vec<u32> = image
        .data()
        .chunks_exact(3)
        .map(|pixel| {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;
            (r << 16) | (g << 8) | b
        })
        .collect();
    
    // Update and control window
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, image.width(), image.height())?;
    }
    
    Ok(())
}
```

## Examples

See the following examples for more demonstrations:

- `examples/simple_imshow.rs` - Basic usage
- `examples/window_display_example.rs` - Comprehensive examples with gradients and patterns

Run examples with:
```bash
cargo run --example simple_imshow --features window
cargo run --example window_display_example --features window
```

## Troubleshooting

### "This example requires the 'window' feature"
Enable the window feature: `cargo run --example simple_imshow --features window`

### "Failed to create window"
- Ensure you have GUI support (not running in headless environment)
- On Linux, ensure X11 or Wayland is available
- Check that image dimensions are valid (> 0)

### Window doesn't appear
- Check that the image has valid dimensions
- Ensure the program isn't terminating immediately after the imshow call
- Try adding error handling to see if an error is being silently ignored

### Image colors look wrong
- Ensure your image data is in RGB format (not BGR)
- cv-rusty uses RGB ordering: `[R, G, B, R, G, B, ...]`
