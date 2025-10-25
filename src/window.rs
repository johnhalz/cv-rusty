//! Window display module for showing images in GUI windows.
//!
//! This module provides functionality for displaying images in GUI windows.
//! It requires the `window` feature to be enabled.
//!
//! # Examples
//!
//! ```no_run
//! use cv_rusty::{Matrix3, Matrix1, show_image};
//!
//! // Works with color images
//! let color_image = Matrix3::zeros(640, 480);
//! show_image("Color Window", &color_image).expect("Failed to display image");
//!
//! // Works with grayscale images
//! let gray_image = Matrix1::zeros(640, 480);
//! show_image("Grayscale Window", &gray_image).expect("Failed to display image");
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

/// Trait for types that can be displayed in a window.
///
/// This trait is implemented for both `Matrix1` (grayscale) and `Matrix3` (color) images,
/// allowing the `show_image` function to work with either type.
pub trait Displayable {
    /// Converts the image to a buffer format suitable for display.
    ///
    /// Returns a tuple of (buffer, width, height).
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError>;
}

impl Displayable for Matrix1 {
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError> {
        let width = self.width();
        let height = self.height();

        if width == 0 || height == 0 {
            return Err(WindowError::InvalidDimensions);
        }

        // Convert grayscale to RGB buffer (minifb expects u32 RGB format: 0x00RRGGBB)
        let buffer: Vec<u32> = self
            .data()
            .iter()
            .map(|&pixel| {
                let rgb = pixel as u32;
                (rgb << 16) | (rgb << 8) | rgb
            })
            .collect();

        Ok((buffer, width, height))
    }
}

impl Displayable for Matrix3 {
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError> {
        let width = self.width();
        let height = self.height();

        if width == 0 || height == 0 {
            return Err(WindowError::InvalidDimensions);
        }

        // Convert RGB to minifb's u32 format (0x00RRGGBB)
        let buffer: Vec<u32> = self
            .data()
            .chunks_exact(3)
            .map(|pixel| {
                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                (r << 16) | (g << 8) | b
            })
            .collect();

        Ok((buffer, width, height))
    }
}

/// Displays an image in a window.
///
/// Creates or updates a window with the given name and displays the image.
/// Works with both grayscale (`Matrix1`) and color (`Matrix3`) images.
/// The window will remain open until closed by the user or until ESC is pressed.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create or update
/// * `image` - The image to display (can be `Matrix1` or `Matrix3`)
///
/// # Returns
///
/// Returns `Ok(())` if the image was displayed successfully, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix1, Matrix3, show_image};
///
/// // Display a color image
/// let color_image = Matrix3::zeros(640, 480);
/// show_image("Color Window", &color_image).expect("Failed to display image");
///
/// // Display a grayscale image
/// let gray_image = Matrix1::zeros(640, 480);
/// show_image("Grayscale Window", &gray_image).expect("Failed to display image");
/// ```
pub fn show_image<T: Displayable>(window_name: &str, image: &T) -> Result<(), WindowError> {
    let (buffer, width, height) = image.to_display_buffer()?;
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
/// This function blocks execution and waits for a key press.
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
/// use cv_rusty::{Matrix3, show_image, wait_key};
///
/// let image = Matrix3::zeros(640, 480);
/// show_image("My Window", &image).expect("Failed to display image");
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

/// Displays an image and waits for user interaction, then closes the window.
///
/// This is a convenience function that displays the image and blocks until
/// the user presses ESC or closes the window. Works with both color and grayscale images.
///
/// # Arguments
///
/// * `window_name` - The name of the window to create
/// * `image` - The image to display (can be `Matrix1` or `Matrix3`)
///
/// # Returns
///
/// Returns `Ok(())` if successful, or a `WindowError` if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, Matrix1, show_and_wait};
///
/// // Works with color images
/// let color_image = Matrix3::zeros(640, 480);
/// show_and_wait("Color Window", &color_image).expect("Failed to display image");
///
/// // Works with grayscale images
/// let gray_image = Matrix1::zeros(640, 480);
/// show_and_wait("Grayscale Window", &gray_image).expect("Failed to display image");
/// ```
pub fn show_and_wait<T: Displayable>(window_name: &str, image: &T) -> Result<(), WindowError> {
    show_image(window_name, image)
}
