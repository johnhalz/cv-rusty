//! Color space conversion module.
//!
//! This module provides functions for converting between different color spaces
//! and converting multi-channel images to single-channel grayscale images.
//!
//! This module is `no_std` compatible and only requires the `alloc` crate.

#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::matrix::{Matrix1, Matrix3};

/// Methods for converting RGB images to grayscale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrayscaleMethod {
    /// Luminosity method (weighted average): 0.299*R + 0.587*G + 0.114*B
    /// This method accounts for human perception where green appears brightest.
    Luminosity,
    /// Simple average method: (R + G + B) / 3
    Average,
    /// Lightness method: (max(R,G,B) + min(R,G,B)) / 2
    Lightness,
}

impl Matrix3 {
    /// Converts an RGB image to grayscale using the luminosity method.
    ///
    /// This is the default and recommended method as it accounts for human perception.
    /// The formula is: 0.299*R + 0.587*G + 0.114*B
    ///
    /// # Returns
    ///
    /// A single-channel Matrix1 containing the grayscale image.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::Matrix3;
    ///
    /// let mut rgb_image = Matrix3::zeros(100, 100);
    /// let gray_image = rgb_image.to_grayscale();
    /// ```
    pub fn to_grayscale(&self) -> Matrix1 {
        self.to_grayscale_with_method(GrayscaleMethod::Luminosity)
    }

    /// Converts an RGB image to grayscale using the average method.
    ///
    /// The formula is: (R + G + B) / 3
    ///
    /// # Returns
    ///
    /// A single-channel Matrix1 containing the grayscale image.
    pub fn to_grayscale_average(&self) -> Matrix1 {
        self.to_grayscale_with_method(GrayscaleMethod::Average)
    }

    /// Converts an RGB image to grayscale using the lightness method.
    ///
    /// The formula is: (max(R,G,B) + min(R,G,B)) / 2
    ///
    /// # Returns
    ///
    /// A single-channel Matrix1 containing the grayscale image.
    pub fn to_grayscale_lightness(&self) -> Matrix1 {
        self.to_grayscale_with_method(GrayscaleMethod::Lightness)
    }

    /// Converts an RGB image to grayscale using the specified method.
    ///
    /// # Arguments
    ///
    /// * `method` - The grayscale conversion method to use
    ///
    /// # Returns
    ///
    /// A single-channel Matrix1 containing the grayscale image.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix3, GrayscaleMethod};
    ///
    /// let mut rgb_image = Matrix3::zeros(100, 100);
    /// let gray_image = rgb_image.to_grayscale_with_method(GrayscaleMethod::Average);
    /// ```
    pub fn to_grayscale_with_method(&self, method: GrayscaleMethod) -> Matrix1 {
        let mut gray_data = vec![0u8; self.width() * self.height()];

        match method {
            GrayscaleMethod::Luminosity => {
                for i in 0..self.width() * self.height() {
                    let rgb_idx = i * 3;
                    let r = self.data()[rgb_idx] as f32;
                    let g = self.data()[rgb_idx + 1] as f32;
                    let b = self.data()[rgb_idx + 2] as f32;
                    gray_data[i] = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                }
            }
            GrayscaleMethod::Average => {
                for i in 0..self.width() * self.height() {
                    let rgb_idx = i * 3;
                    let r = self.data()[rgb_idx] as u16;
                    let g = self.data()[rgb_idx + 1] as u16;
                    let b = self.data()[rgb_idx + 2] as u16;
                    gray_data[i] = ((r + g + b) / 3) as u8;
                }
            }
            GrayscaleMethod::Lightness => {
                for i in 0..self.width() * self.height() {
                    let rgb_idx = i * 3;
                    let r = self.data()[rgb_idx];
                    let g = self.data()[rgb_idx + 1];
                    let b = self.data()[rgb_idx + 2];
                    let max = r.max(g).max(b);
                    let min = r.min(g).min(b);
                    gray_data[i] = ((max as u16 + min as u16) / 2) as u8;
                }
            }
        }

        Matrix1::new(self.width(), self.height(), gray_data)
    }
}

/// Converts RGB color values to HSV (Hue, Saturation, Value) color space.
///
/// # Arguments
///
/// * `r` - Red channel value (0-255)
/// * `g` - Green channel value (0-255)
/// * `b` - Blue channel value (0-255)
///
/// # Returns
///
/// A tuple (h, s, v) where:
/// - h (hue) is in degrees (0.0-360.0)
/// - s (saturation) is in range (0.0-1.0)
/// - v (value) is in range (0.0-1.0)
///
/// # Examples
///
/// ```
/// use cv_rusty::rgb_to_hsv;
///
/// let (h, s, v) = rgb_to_hsv(255, 0, 0); // Pure red
/// assert!((h - 0.0).abs() < 0.1);
/// assert!((s - 1.0).abs() < 0.1);
/// assert!((v - 1.0).abs() < 0.1);
/// ```
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    let s = if max == 0.0 { 0.0 } else { delta / max };

    let v = max;

    (h, s, v)
}

/// Converts HSV (Hue, Saturation, Value) color values to RGB color space.
///
/// # Arguments
///
/// * `h` - Hue in degrees (0.0-360.0)
/// * `s` - Saturation (0.0-1.0)
/// * `v` - Value (0.0-1.0)
///
/// # Returns
///
/// A tuple (r, g, b) where each component is in range (0-255)
///
/// # Examples
///
/// ```
/// use cv_rusty::hsv_to_rgb;
///
/// let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0); // Pure red
/// assert_eq!(r, 255);
/// assert_eq!(g, 0);
/// assert_eq!(b, 0);
/// ```
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    (r, g, b)
}

/// Converts RGB color values to HSL (Hue, Saturation, Lightness) color space.
///
/// # Arguments
///
/// * `r` - Red channel value (0-255)
/// * `g` - Green channel value (0-255)
/// * `b` - Blue channel value (0-255)
///
/// # Returns
///
/// A tuple (h, s, l) where:
/// - h (hue) is in degrees (0.0-360.0)
/// - s (saturation) is in range (0.0-1.0)
/// - l (lightness) is in range (0.0-1.0)
///
/// # Examples
///
/// ```
/// use cv_rusty::rgb_to_hsl;
///
/// let (h, s, l) = rgb_to_hsl(255, 0, 0); // Pure red
/// assert!((h - 0.0).abs() < 0.1);
/// assert!((s - 1.0).abs() < 0.1);
/// assert!((l - 0.5).abs() < 0.1);
/// ```
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    (h, s, l)
}

/// Converts HSL (Hue, Saturation, Lightness) color values to RGB color space.
///
/// # Arguments
///
/// * `h` - Hue in degrees (0.0-360.0)
/// * `s` - Saturation (0.0-1.0)
/// * `l` - Lightness (0.0-1.0)
///
/// # Returns
///
/// A tuple (r, g, b) where each component is in range (0-255)
///
/// # Examples
///
/// ```
/// use cv_rusty::hsl_to_rgb;
///
/// let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5); // Pure red
/// assert_eq!(r, 255);
/// assert_eq!(g, 0);
/// assert_eq!(b, 0);
/// ```
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsv_pure_colors() {
        // Red
        let (h, s, v) = rgb_to_hsv(255, 0, 0);
        assert!((h - 0.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        // Green
        let (h, s, v) = rgb_to_hsv(0, 255, 0);
        assert!((h - 120.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);

        // Blue
        let (h, s, v) = rgb_to_hsv(0, 0, 255);
        assert!((h - 240.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_hsv_to_rgb_pure_colors() {
        // Red
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Green
        let (r, g, b) = hsv_to_rgb(120.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);

        // Blue
        let (r, g, b) = hsv_to_rgb(240.0, 1.0, 1.0);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_rgb_hsv_roundtrip() {
        let test_colors = vec![
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (255, 0, 255),
            (0, 255, 255),
            (128, 64, 192),
        ];

        for (r, g, b) in test_colors {
            let (h, s, v) = rgb_to_hsv(r, g, b);
            let (r2, g2, b2) = hsv_to_rgb(h, s, v);
            assert!(
                (r as i16 - r2 as i16).abs() <= 1,
                "Red mismatch: {} vs {}",
                r,
                r2
            );
            assert!(
                (g as i16 - g2 as i16).abs() <= 1,
                "Green mismatch: {} vs {}",
                g,
                g2
            );
            assert!(
                (b as i16 - b2 as i16).abs() <= 1,
                "Blue mismatch: {} vs {}",
                b,
                b2
            );
        }
    }

    #[test]
    fn test_rgb_to_hsl_pure_colors() {
        // Red
        let (h, s, l) = rgb_to_hsl(255, 0, 0);
        assert!((h - 0.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);

        // Green
        let (h, s, l) = rgb_to_hsl(0, 255, 0);
        assert!((h - 120.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);

        // Blue
        let (h, s, l) = rgb_to_hsl(0, 0, 255);
        assert!((h - 240.0).abs() < 0.1);
        assert!((s - 1.0).abs() < 0.01);
        assert!((l - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_hsl_to_rgb_pure_colors() {
        // Red
        let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Green
        let (r, g, b) = hsl_to_rgb(120.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);

        // Blue
        let (r, g, b) = hsl_to_rgb(240.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_rgb_hsl_roundtrip() {
        let test_colors = vec![
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (255, 255, 0),
            (255, 0, 255),
            (0, 255, 255),
            (128, 64, 192),
        ];

        for (r, g, b) in test_colors {
            let (h, s, l) = rgb_to_hsl(r, g, b);
            let (r2, g2, b2) = hsl_to_rgb(h, s, l);
            assert!(
                (r as i16 - r2 as i16).abs() <= 1,
                "Red mismatch: {} vs {}",
                r,
                r2
            );
            assert!(
                (g as i16 - g2 as i16).abs() <= 1,
                "Green mismatch: {} vs {}",
                g,
                g2
            );
            assert!(
                (b as i16 - b2 as i16).abs() <= 1,
                "Blue mismatch: {} vs {}",
                b,
                b2
            );
        }
    }

    #[test]
    fn test_grayscale_methods() {
        let mut mat = Matrix3::zeros(2, 2);
        mat.set_pixel(0, 0, 255, 0, 0);
        mat.set_pixel(1, 0, 0, 255, 0);
        mat.set_pixel(0, 1, 0, 0, 255);
        mat.set_pixel(1, 1, 255, 255, 255);

        // Test all three methods
        let gray_lum = mat.to_grayscale();
        let gray_avg = mat.to_grayscale_average();
        let gray_light = mat.to_grayscale_lightness();

        assert_eq!(gray_lum.width(), 2);
        assert_eq!(gray_avg.width(), 2);
        assert_eq!(gray_light.width(), 2);

        // White should be 255 in all methods
        assert_eq!(gray_avg.get_pixel(1, 1), Some(255));
        assert_eq!(gray_light.get_pixel(1, 1), Some(255));
    }
}
