//! Window display module for showing images in GUI windows.
//!
//! This module provides functionality similar to OpenCV's `imshow` and `waitKey`
//! for displaying images in windows. It requires the `window` feature to be enabled.
//!
//! # Examples
//!
//! ```no_run
//! use cv_rusty::{Matrix3, imshow, wait_key};
//!
//! let image = Matrix3::zeros(640, 480);
//! imshow("My Window", &image).expect("Failed to display image");
//! wait_key(0); // Wait indefinitely for a key press
//! ```

use crate::{Matrix1, Matrix3};
use minifb::{Key, Window, WindowOptions};
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// Error type for window operations.
#[derive(Debug)]
pub enum WindowError {
    /// Failed to create or update a window
    WindowCreation(String),
    /// Invalid image dimensions
    InvalidDimensions,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowError::WindowCreation(msg) => write!(f, "Window error: {}", msg),
            WindowError::InvalidDimensions => write!(f, "Invalid image dimensions"),
        }
    }
}

impl Error for WindowError {}

/// Displays a grayscale image (Matrix1) in a window.
///
/// Similar to OpenCV's `imshow`, this function creates or updates a window with the given name
/// and displays the image. The window will remain open until closed by the user or until
/// `wait_key` is called.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create or update
/// * `image` - The grayscale image to display
///
/// # Returns
///
/// Returns `Ok(())` if the image was displayed successfully, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix1, imshow};
///
/// let image = Matrix1::zeros(640, 480);
/// imshow("Grayscale Image", &image).expect("Failed to display image");
/// ```
pub fn imshow(window_name: &str, image: &Matrix1) -> Result<(), WindowError> {
    let width = image.width();
    let height = image.height();

    if width == 0 || height == 0 {
        return Err(WindowError::InvalidDimensions);
    }

    // Convert grayscale to RGB buffer (minifb expects u32 RGB format)
    let buffer: Vec<u32> = image
        .data()
        .iter()
        .map(|&pixel| {
            // Convert grayscale to RGB: 0x00RRGGBB
            let rgb = pixel as u32;
            (rgb << 16) | (rgb << 8) | rgb
        })
        .collect();

    display_buffer(window_name, &buffer, width, height)
}

/// Displays a color image (Matrix3) in a window.
///
/// Similar to OpenCV's `imshow`, this function creates or updates a window with the given name
/// and displays the RGB image. The window will remain open until closed by the user or until
/// `wait_key` is called.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create or update
/// * `image` - The RGB image to display
///
/// # Returns
///
/// Returns `Ok(())` if the image was displayed successfully, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, imshow_color};
///
/// let image = Matrix3::zeros(640, 480);
/// imshow_color("Color Image", &image).expect("Failed to display image");
/// ```
pub fn imshow_color(window_name: &str, image: &Matrix3) -> Result<(), WindowError> {
    let width = image.width();
    let height = image.height();

    if width == 0 || height == 0 {
        return Err(WindowError::InvalidDimensions);
    }

    // Convert RGB to minifb's u32 format (0x00RRGGBB)
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

    display_buffer(window_name, &buffer, width, height)
}

/// Internal function to display a buffer in a window.
fn display_buffer(
    window_name: &str,
    buffer: &[u32],
    width: usize,
    height: usize,
) -> Result<(), WindowError> {
    let mut window = Window::new(window_name, width, height, WindowOptions::default())
        .map_err(|e| WindowError::WindowCreation(e.to_string()))?;

    // Limit to max ~60 fps
    window.set_target_fps(60);

    // Keep the window open and responsive
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(buffer, width, height)
            .map_err(|e| WindowError::WindowCreation(e.to_string()))?;
    }

    Ok(())
}

/// Waits for a key press for a specified duration.
///
/// Similar to OpenCV's `waitKey`, this function blocks execution and waits for a key press.
/// If `delay` is 0, it waits indefinitely. Otherwise, it waits for the specified number
/// of milliseconds.
///
/// This is a simplified version that just sleeps for the specified duration. In a real
/// application with multiple windows, you would need more sophisticated event handling.
///
/// # Arguments
///
/// * `delay` - The number of milliseconds to wait. Use 0 to wait indefinitely.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, imshow_color, wait_key};
///
/// let image = Matrix3::zeros(640, 480);
/// imshow_color("My Window", &image).expect("Failed to display image");
/// wait_key(1000); // Wait for 1 second
/// ```
pub fn wait_key(delay: u64) {
    if delay == 0 {
        // Wait indefinitely - in practice, sleep for a very long time
        std::thread::sleep(Duration::from_secs(u64::MAX));
    } else {
        std::thread::sleep(Duration::from_millis(delay));
    }
}

/// Displays an image and waits for a key press, then closes the window.
///
/// This is a convenience function that combines `imshow_color` and a blocking wait.
/// The window will close when the user presses ESC or closes the window.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create
/// * `image` - The RGB image to display
///
/// # Returns
///
/// Returns `Ok(())` if successful, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, show_and_wait};
///
/// let image = Matrix3::zeros(640, 480);
/// show_and_wait("My Window", &image).expect("Failed to display image");
/// ```
pub fn show_and_wait(window_name: &str, image: &Matrix3) -> Result<(), WindowError> {
    imshow_color(window_name, image)
}

/// Displays a grayscale image and waits for a key press, then closes the window.
///
/// This is a convenience function that combines `imshow` and a blocking wait.
/// The window will close when the user presses ESC or closes the window.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create
/// * `image` - The grayscale image to display
///
/// # Returns
///
/// Returns `Ok(())` if successful, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix1, show_and_wait_gray};
///
/// let image = Matrix1::zeros(640, 480);
/// show_and_wait_gray("My Window", &image).expect("Failed to display image");
/// ```
pub fn show_and_wait_gray(window_name: &str, image: &Matrix1) -> Result<(), WindowError> {
    imshow(window_name, image)
}
