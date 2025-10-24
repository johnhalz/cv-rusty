//! Matrix module for representing multi-channel image data.
//!
//! This module is `no_std` compatible and only requires the `alloc` crate.

#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::fmt;

/// A three-channel matrix for representing RGB image data.
///
/// The data is stored in a contiguous Vec<u8> in row-major order,
/// with channels interleaved (RGBRGBRGB...).
#[derive(Debug, Clone)]
pub struct Matrix3 {
    /// Width of the matrix (number of columns)
    width: usize,
    /// Height of the matrix (number of rows)
    height: usize,
    /// Raw pixel data stored as [R, G, B, R, G, B, ...]
    data: Vec<u8>,
}

impl Matrix3 {
    /// Creates a new Matrix3 with the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `width` - The width (number of columns) of the matrix
    /// * `height` - The height (number of rows) of the matrix
    /// * `data` - The raw pixel data in RGB format (must be width * height * 3 bytes)
    ///
    /// # Panics
    ///
    /// Panics if the data length doesn't match width * height * 3.
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        assert_eq!(
            data.len(),
            width * height * 3,
            "Data length must be width * height * 3"
        );
        Self {
            width,
            height,
            data,
        }
    }

    /// Creates a new Matrix3 filled with zeros.
    ///
    /// # Arguments
    ///
    /// * `width` - The width (number of columns) of the matrix
    /// * `height` - The height (number of rows) of the matrix
    pub fn zeros(width: usize, height: usize) -> Self {
        let data = vec![0u8; width * height * 3];
        Self {
            width,
            height,
            data,
        }
    }

    /// Returns the width of the matrix.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the matrix.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the dimensions as (width, height).
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns a reference to the raw pixel data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns a mutable reference to the raw pixel data.
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Gets the RGB values at the specified pixel location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate (column)
    /// * `y` - The y-coordinate (row)
    ///
    /// # Returns
    ///
    /// Returns Some((r, g, b)) if the coordinates are valid, None otherwise.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = (y * self.width + x) * 3;
        Some((self.data[idx], self.data[idx + 1], self.data[idx + 2]))
    }

    /// Sets the RGB values at the specified pixel location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate (column)
    /// * `y` - The y-coordinate (row)
    /// * `r` - Red channel value
    /// * `g` - Green channel value
    /// * `b` - Blue channel value
    ///
    /// # Returns
    ///
    /// Returns true if the pixel was set successfully, false if coordinates are out of bounds.
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let idx = (y * self.width + x) * 3;
        self.data[idx] = r;
        self.data[idx + 1] = g;
        self.data[idx + 2] = b;
        true
    }

    /// Consumes the matrix and returns the raw data.
    pub fn into_raw(self) -> Vec<u8> {
        self.data
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix3 {{ width: {}, height: {}, channels: 3 }}",
            self.width, self.height
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_matrix() {
        let data = vec![0u8; 100 * 100 * 3];
        let mat = Matrix3::new(100, 100, data);
        assert_eq!(mat.width(), 100);
        assert_eq!(mat.height(), 100);
        assert_eq!(mat.data().len(), 100 * 100 * 3);
    }

    #[test]
    fn test_zeros() {
        let mat = Matrix3::zeros(50, 50);
        assert_eq!(mat.width(), 50);
        assert_eq!(mat.height(), 50);
        assert!(mat.data().iter().all(|&x| x == 0));
    }

    #[test]
    fn test_get_set_pixel() {
        let mut mat = Matrix3::zeros(10, 10);
        assert!(mat.set_pixel(5, 5, 255, 128, 64));
        assert_eq!(mat.get_pixel(5, 5), Some((255, 128, 64)));
        assert_eq!(mat.get_pixel(10, 10), None);
    }

    #[test]
    #[should_panic]
    fn test_new_invalid_size() {
        let data = vec![0u8; 100];
        Matrix3::new(10, 10, data); // Should panic: 100 != 10 * 10 * 3
    }
}
