//! I/O module for reading and writing image files.
//!
//! This module requires the `std` feature to be enabled.

use crate::matrix::Matrix3;
use jpeg_decoder::{Decoder, PixelFormat};
use png::{ColorType, Decoder as PngDecoder};
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
    /// PNG decoding error
    PngDecode(String),
    /// JPEG encoding error
    JpegEncode(String),
    /// PNG encoding error
    PngEncode(String),
    /// Unsupported pixel format
    UnsupportedFormat(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::Io(e) => write!(f, "I/O error: {}", e),
            ImageError::JpegDecode(e) => write!(f, "JPEG decode error: {}", e),
            ImageError::PngDecode(e) => write!(f, "PNG decode error: {}", e),
            ImageError::JpegEncode(e) => write!(f, "JPEG encode error: {}", e),
            ImageError::PngEncode(e) => write!(f, "PNG encode error: {}", e),
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

/// Reads a PNG image file and returns it as a three-channel RGB matrix.
///
/// # Arguments
///
/// * `path` - Path to the PNG file
///
/// # Returns
///
/// Returns a `Result` containing a `Matrix3` with RGB data on success,
/// or an `ImageError` on failure.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::io::read_png;
///
/// let image = read_png("photo.png").expect("Failed to read PNG");
/// println!("Image dimensions: {}x{}", image.width(), image.height());
/// ```
pub fn read_png<P: AsRef<Path>>(path: P) -> Result<Matrix3, ImageError> {
    // Open the file
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Create decoder
    let decoder = PngDecoder::new(reader);
    let mut reader = decoder
        .read_info()
        .map_err(|e| ImageError::PngDecode(format!("{}", e)))?;

    // Get image metadata
    let info = reader.info();
    let width = info.width as usize;
    let height = info.height as usize;
    let color_type = info.color_type;

    // Allocate buffer for image data
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|e| ImageError::PngDecode(format!("{}", e)))?;

    // Resize buffer to actual data size
    buf.truncate(info.buffer_size());

    // Convert to RGB if necessary
    let rgb_data = match color_type {
        ColorType::Rgb => {
            // Already in RGB format
            buf
        }
        ColorType::Rgba => {
            // RGBA - strip alpha channel
            let mut rgb = Vec::with_capacity(width * height * 3);
            for chunk in buf.chunks_exact(4) {
                rgb.push(chunk[0]);
                rgb.push(chunk[1]);
                rgb.push(chunk[2]);
            }
            rgb
        }
        ColorType::Grayscale => {
            // Grayscale - convert to RGB by duplicating the channel
            let mut rgb = Vec::with_capacity(buf.len() * 3);
            for &gray in &buf {
                rgb.push(gray);
                rgb.push(gray);
                rgb.push(gray);
            }
            rgb
        }
        ColorType::GrayscaleAlpha => {
            // Grayscale with alpha - convert to RGB and strip alpha
            let mut rgb = Vec::with_capacity(width * height * 3);
            for chunk in buf.chunks_exact(2) {
                let gray = chunk[0];
                rgb.push(gray);
                rgb.push(gray);
                rgb.push(gray);
            }
            rgb
        }
        ColorType::Indexed => {
            // Indexed color should be expanded by the decoder
            return Err(ImageError::UnsupportedFormat(
                "Indexed PNG color type not fully supported. Try converting to RGB first."
                    .to_string(),
            ));
        }
    };

    Ok(Matrix3::new(width, height, rgb_data))
}

/// Writes a Matrix3 as a JPEG image file.
///
/// # Arguments
///
/// * `matrix` - The Matrix3 containing RGB data to write
/// * `path` - Path where the JPEG file should be written
/// * `quality` - JPEG quality (1-100, where 100 is best quality)
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `ImageError` on failure.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, io::write_jpeg};
///
/// let image = Matrix3::zeros(640, 480);
/// write_jpeg(&image, "output.jpg", 90).expect("Failed to write JPEG");
/// ```
pub fn write_jpeg<P: AsRef<Path>>(
    matrix: &Matrix3,
    path: P,
    quality: u8,
) -> Result<(), ImageError> {
    use jpeg_encoder::{ColorType as JpegColorType, Encoder};

    let quality = quality.clamp(1, 100);

    // Create the output file
    let file = File::create(path)?;
    let mut writer = io::BufWriter::new(file);

    // Create encoder
    let encoder = Encoder::new(&mut writer, quality);

    // Encode the image
    encoder
        .encode(
            matrix.data(),
            matrix.width() as u16,
            matrix.height() as u16,
            JpegColorType::Rgb,
        )
        .map_err(|e| ImageError::JpegEncode(format!("{}", e)))?;

    Ok(())
}

/// Writes a Matrix3 as a PNG image file.
///
/// # Arguments
///
/// * `matrix` - The Matrix3 containing RGB data to write
/// * `path` - Path where the PNG file should be written
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `ImageError` on failure.
///
/// # Examples
///
/// ```no_run
/// use cv_rusty::{Matrix3, io::write_png};
///
/// let image = Matrix3::zeros(640, 480);
/// write_png(&image, "output.png").expect("Failed to write PNG");
/// ```
pub fn write_png<P: AsRef<Path>>(matrix: &Matrix3, path: P) -> Result<(), ImageError> {
    use png::{BitDepth, Encoder};

    // Create the output file
    let file = File::create(path)?;
    let writer = io::BufWriter::new(file);

    // Create encoder
    let mut encoder = Encoder::new(writer, matrix.width() as u32, matrix.height() as u32);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);

    // Write the PNG header
    let mut writer = encoder
        .write_header()
        .map_err(|e| ImageError::PngEncode(format!("{}", e)))?;

    // Write the image data
    writer
        .write_image_data(matrix.data())
        .map_err(|e| ImageError::PngEncode(format!("{}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_image_error_display() {
        let err = ImageError::JpegDecode("test error".to_string());
        assert_eq!(format!("{}", err), "JPEG decode error: test error");

        let err = ImageError::PngDecode("png error".to_string());
        assert_eq!(format!("{}", err), "PNG decode error: png error");

        let err = ImageError::JpegEncode("encode error".to_string());
        assert_eq!(format!("{}", err), "JPEG encode error: encode error");

        let err = ImageError::PngEncode("encode error".to_string());
        assert_eq!(format!("{}", err), "PNG encode error: encode error");
    }

    #[test]
    fn test_unsupported_format_error() {
        let err = ImageError::UnsupportedFormat("RGBA".to_string());
        assert!(format!("{}", err).contains("Unsupported format"));
    }

    #[test]
    fn test_write_and_read_jpeg() {
        // Create a test image with a gradient pattern
        let width = 100;
        let height = 100;
        let mut data = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            for x in 0..width {
                data.push((x * 255 / width) as u8);
                data.push((y * 255 / height) as u8);
                data.push(128);
            }
        }

        let original = Matrix3::new(width, height, data);

        // Write JPEG
        let temp_path = "test_output.jpg";
        write_jpeg(&original, temp_path, 90).expect("Failed to write JPEG");

        // Read it back
        let loaded = read_jpeg(temp_path).expect("Failed to read JPEG");

        // Verify dimensions
        assert_eq!(loaded.width(), original.width());
        assert_eq!(loaded.height(), original.height());

        // Clean up
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_write_and_read_png() {
        // Create a test image with a specific pattern
        let width = 50;
        let height = 50;
        let mut data = Vec::with_capacity(width * height * 3);

        for y in 0..height {
            for x in 0..width {
                data.push((x * 5) as u8);
                data.push((y * 5) as u8);
                data.push(200);
            }
        }

        let original = Matrix3::new(width, height, data.clone());

        // Write PNG
        let temp_path = "test_output.png";
        write_png(&original, temp_path).expect("Failed to write PNG");

        // Read it back
        let loaded = read_png(temp_path).expect("Failed to read PNG");

        // Verify dimensions
        assert_eq!(loaded.width(), original.width());
        assert_eq!(loaded.height(), original.height());

        // PNG is lossless, so data should match exactly
        assert_eq!(loaded.data(), &data[..]);

        // Clean up
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_write_jpeg_quality_bounds() {
        let image = Matrix3::zeros(10, 10);
        let temp_path = "test_quality.jpg";

        // Test with quality values outside bounds - should clamp
        write_jpeg(&image, temp_path, 0).expect("Should clamp to 1");
        write_jpeg(&image, temp_path, 150).expect("Should clamp to 100");

        // Clean up
        fs::remove_file(temp_path).ok();
    }
}
