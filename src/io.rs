//! I/O module for reading and writing image files.
//!
//! This module requires the `std` feature to be enabled.

use crate::matrix::Matrix3;
use jpeg_decoder::{Decoder, PixelFormat};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

/// Errors that can occur during image I/O operations.
#[derive(Debug)]
pub enum ImageError {
    /// I/O error occurred
    Io(io::Error),
    /// JPEG decoding error
    JpegDecode(String),
    /// Unsupported pixel format
    UnsupportedFormat(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::Io(e) => write!(f, "I/O error: {}", e),
            ImageError::JpegDecode(e) => write!(f, "JPEG decode error: {}", e),
            ImageError::UnsupportedFormat(e) => write!(f, "Unsupported format: {}", e),
        }
    }
}

impl std::error::Error for ImageError {}

impl From<io::Error> for ImageError {
    fn from(error: io::Error) -> Self {
        ImageError::Io(error)
    }
}

/// Reads a JPEG image file and returns it as a three-channel RGB matrix.
///
/// # Arguments
///
/// * `path` - Path to the JPEG file
///
/// # Returns
///
/// Returns a `Result` containing a `Matrix3` with RGB data on success,
/// or an `ImageError` on failure.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::io::read_jpeg;
///
/// let image = read_jpeg("photo.jpg").expect("Failed to read JPEG");
/// println!("Image dimensions: {}x{}", image.width(), image.height());
/// ```
pub fn read_jpeg<P: AsRef<Path>>(path: P) -> Result<Matrix3, ImageError> {
    // Open the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Create decoder
    let mut decoder = Decoder::new(reader);

    // Decode the image
    let pixels = decoder
        .decode()
        .map_err(|e| ImageError::JpegDecode(format!("{:?}", e)))?;

    // Get image metadata
    let info = decoder
        .info()
        .ok_or_else(|| ImageError::JpegDecode("Failed to get image info".to_string()))?;

    let width = info.width as usize;
    let height = info.height as usize;

    // Convert to RGB if necessary
    let rgb_data = match info.pixel_format {
        PixelFormat::RGB24 => {
            // Already in RGB format
            pixels
        }
        PixelFormat::L8 => {
            // Grayscale - convert to RGB by duplicating the channel
            let mut rgb = Vec::with_capacity(pixels.len() * 3);
            for &gray in &pixels {
                rgb.push(gray);
                rgb.push(gray);
                rgb.push(gray);
            }
            rgb
        }
        PixelFormat::CMYK32 => {
            // CMYK - convert to RGB
            let mut rgb = Vec::with_capacity((width * height * 3) as usize);
            for chunk in pixels.chunks_exact(4) {
                let c = chunk[0] as f32 / 255.0;
                let m = chunk[1] as f32 / 255.0;
                let y = chunk[2] as f32 / 255.0;
                let k = chunk[3] as f32 / 255.0;

                let r = ((1.0 - c) * (1.0 - k) * 255.0) as u8;
                let g = ((1.0 - m) * (1.0 - k) * 255.0) as u8;
                let b = ((1.0 - y) * (1.0 - k) * 255.0) as u8;

                rgb.push(r);
                rgb.push(g);
                rgb.push(b);
            }
            rgb
        }
        _ => {
            return Err(ImageError::UnsupportedFormat(format!(
                "Unsupported pixel format: {:?}",
                info.pixel_format
            )));
        }
    };

    Ok(Matrix3::new(width, height, rgb_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_error_display() {
        let err = ImageError::JpegDecode("test error".to_string());
        assert_eq!(format!("{}", err), "JPEG decode error: test error");
    }

    #[test]
    fn test_unsupported_format_error() {
        let err = ImageError::UnsupportedFormat("RGBA".to_string());
        assert!(format!("{}", err).contains("Unsupported format"));
    }
}
