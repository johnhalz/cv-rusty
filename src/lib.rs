//! CV Rusty - A no_std computer vision library for Rust
//!
//! This library provides computer vision functionality optimized for live computations
//! and embedded systems. The core library is `no_std` compatible and only requires `alloc`.
//!
//! # Features
//!
//! - `std` (default): Enables standard library support and file I/O operations
//! - `alloc`: Enables heap allocation support (required for core functionality)
//!
//! # Examples
//!
//! With `std` feature (default):
//! ```no_run
//! use cv_rusty::{read_jpeg, read_png, write_jpeg, write_png, Matrix3};
//!
//! // Read JPEG image
//! let image = read_jpeg("photo.jpg").expect("Failed to read JPEG");
//! println!("Loaded {}x{} image", image.width(), image.height());
//!
//! // Read PNG image
//! let image = read_png("photo.png").expect("Failed to read PNG");
//! println!("Loaded {}x{} image", image.width(), image.height());
//!
//! // Write JPEG image with quality 90
//! let output = Matrix3::zeros(640, 480);
//! write_jpeg(&output, "output.jpg", 90).expect("Failed to write JPEG");
//!
//! // Write PNG image
//! write_png(&output, "output.png").expect("Failed to write PNG");
//! ```
//!
//! Without `std` (no_std + alloc):
//! ```
//! use cv_rusty::Matrix3;
//!
//! let image = Matrix3::zeros(640, 480);
//! // Work with image data in embedded environment
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod color;
pub mod convolution;
pub mod drawing;
pub mod matrix;
pub mod transform;

#[cfg(feature = "std")]
pub mod io;

#[cfg(feature = "window")]
pub mod window;

pub use color::{hsl_to_rgb, hsv_to_rgb, rgb_to_hsl, rgb_to_hsv, GrayscaleMethod};
pub use convolution::{BorderMode, Kernel};
pub use drawing::{draw_circle, draw_rectangle, Color, DrawTarget, HexParseError};
pub use matrix::{Matrix1, Matrix3};
pub use transform::{InterpolationMethod, Rotation, RotationAngle};

#[cfg(feature = "std")]
pub use io::{read_jpeg, read_png, write_jpeg, write_png};

#[cfg(feature = "window")]
pub use window::{show_and_wait, show_image, wait_key, Displayable, WindowError};
