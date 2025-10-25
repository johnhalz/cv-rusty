//! Transform module for image operations like resize, crop, and rotate.
//!
//! This module is `no_std` compatible and only requires the `alloc` crate.

#[cfg(not(feature = "std"))]
use alloc::vec;

use crate::matrix::{Matrix1, Matrix3};
use core::f32::consts::PI;
use libm::{ceilf, cosf, floorf, roundf, sinf};

/// Interpolation method for resizing operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationMethod {
    /// Nearest neighbor interpolation (fastest, lowest quality)
    NearestNeighbor,
    /// Bilinear interpolation (good balance of speed and quality)
    Bilinear,
}

/// Rotation angle in 90-degree increments (fast, lossless).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAngle {
    /// Rotate 90 degrees clockwise
    Rotate90,
    /// Rotate 180 degrees
    Rotate180,
    /// Rotate 270 degrees clockwise (90 degrees counter-clockwise)
    Rotate270,
}

/// Represents a rotation angle with arbitrary value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    /// Rotation in degrees (e.g., 45.0 for 45 degrees clockwise)
    Degrees(f32),
    /// Rotation in radians (e.g., PI/4 for 45 degrees clockwise)
    Radians(f32),
}

impl Rotation {
    /// Converts the rotation to radians.
    pub fn to_radians(&self) -> f32 {
        match self {
            Rotation::Degrees(deg) => deg * PI / 180.0,
            Rotation::Radians(rad) => *rad,
        }
    }

    /// Converts the rotation to degrees.
    pub fn to_degrees(&self) -> f32 {
        match self {
            Rotation::Degrees(deg) => *deg,
            Rotation::Radians(rad) => rad * 180.0 / PI,
        }
    }
}

impl Matrix1 {
    /// Resizes the image to the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `new_width` - Target width
    /// * `new_height` - Target height
    /// * `method` - Interpolation method to use
    ///
    /// # Returns
    ///
    /// A new Matrix1 with the resized image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix1, InterpolationMethod};
    ///
    /// let image = Matrix1::zeros(640, 480);
    /// let resized = image.resize(320, 240, InterpolationMethod::Bilinear);
    /// assert_eq!(resized.width(), 320);
    /// assert_eq!(resized.height(), 240);
    /// ```
    pub fn resize(&self, new_width: usize, new_height: usize, method: InterpolationMethod) -> Self {
        match method {
            InterpolationMethod::NearestNeighbor => self.resize_nearest(new_width, new_height),
            InterpolationMethod::Bilinear => self.resize_bilinear(new_width, new_height),
        }
    }

    /// Resizes using nearest neighbor interpolation.
    fn resize_nearest(&self, new_width: usize, new_height: usize) -> Self {
        let mut data = vec![0u8; new_width * new_height];

        let x_ratio = self.width() as f32 / new_width as f32;
        let y_ratio = self.height() as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio) as usize;
                let src_y = (y as f32 * y_ratio) as usize;

                let src_x = src_x.min(self.width() - 1);
                let src_y = src_y.min(self.height() - 1);

                let src_idx = src_y * self.width() + src_x;
                let dst_idx = y * new_width + x;

                data[dst_idx] = self.data()[src_idx];
            }
        }

        Matrix1::new(new_width, new_height, data)
    }

    /// Resizes using bilinear interpolation.
    fn resize_bilinear(&self, new_width: usize, new_height: usize) -> Self {
        let mut data = vec![0u8; new_width * new_height];

        let x_ratio = (self.width() - 1) as f32 / new_width as f32;
        let y_ratio = (self.height() - 1) as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x as f32 * x_ratio;
                let src_y = y as f32 * y_ratio;

                let x1 = floorf(src_x) as usize;
                let y1 = floorf(src_y) as usize;
                let x2 = (x1 + 1).min(self.width() - 1);
                let y2 = (y1 + 1).min(self.height() - 1);

                let dx = src_x - x1 as f32;
                let dy = src_y - y1 as f32;

                let p11 = self.data()[y1 * self.width() + x1] as f32;
                let p12 = self.data()[y2 * self.width() + x1] as f32;
                let p21 = self.data()[y1 * self.width() + x2] as f32;
                let p22 = self.data()[y2 * self.width() + x2] as f32;

                let val = p11 * (1.0 - dx) * (1.0 - dy)
                    + p21 * dx * (1.0 - dy)
                    + p12 * (1.0 - dx) * dy
                    + p22 * dx * dy;

                data[y * new_width + x] = roundf(val) as u8;
            }
        }

        Matrix1::new(new_width, new_height, data)
    }

    /// Crops the image to the specified rectangle.
    ///
    /// # Arguments
    ///
    /// * `x` - X-coordinate of the top-left corner
    /// * `y` - Y-coordinate of the top-left corner
    /// * `width` - Width of the crop region
    /// * `height` - Height of the crop region
    ///
    /// # Returns
    ///
    /// Returns Some(Matrix1) if the crop region is valid, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::Matrix1;
    ///
    /// let image = Matrix1::zeros(640, 480);
    /// let cropped = image.crop(100, 100, 200, 200).unwrap();
    /// assert_eq!(cropped.width(), 200);
    /// assert_eq!(cropped.height(), 200);
    /// ```
    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Option<Self> {
        // Validate crop region
        if x + width > self.width() || y + height > self.height() {
            return None;
        }

        let mut data = vec![0u8; width * height];

        for row in 0..height {
            let src_start = (y + row) * self.width() + x;
            let dst_start = row * width;
            data[dst_start..dst_start + width]
                .copy_from_slice(&self.data()[src_start..src_start + width]);
        }

        Some(Matrix1::new(width, height, data))
    }

    /// Rotates the image by the specified angle.
    ///
    /// Only 90-degree rotations are supported for efficiency and lossless transformation.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle (90, 180, or 270 degrees)
    ///
    /// # Returns
    ///
    /// A new Matrix1 with the rotated image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix1, RotationAngle};
    ///
    /// let image = Matrix1::zeros(640, 480);
    /// let rotated = image.rotate(RotationAngle::Rotate90);
    /// assert_eq!(rotated.width(), 480);
    /// assert_eq!(rotated.height(), 640);
    /// ```
    pub fn rotate(&self, angle: RotationAngle) -> Self {
        match angle {
            RotationAngle::Rotate90 => self.rotate_90(),
            RotationAngle::Rotate180 => self.rotate_180(),
            RotationAngle::Rotate270 => self.rotate_270(),
        }
    }

    /// Rotates the image 90 degrees clockwise.
    fn rotate_90(&self) -> Self {
        let new_width = self.height();
        let new_height = self.width();
        let mut data = vec![0u8; new_width * new_height];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = y * self.width() + x;
                let new_x = self.height() - 1 - y;
                let new_y = x;
                let dst_idx = new_y * new_width + new_x;
                data[dst_idx] = self.data()[src_idx];
            }
        }

        Matrix1::new(new_width, new_height, data)
    }

    /// Rotates the image 180 degrees.
    fn rotate_180(&self) -> Self {
        let mut data = vec![0u8; self.width() * self.height()];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = y * self.width() + x;
                let new_x = self.width() - 1 - x;
                let new_y = self.height() - 1 - y;
                let dst_idx = new_y * self.width() + new_x;
                data[dst_idx] = self.data()[src_idx];
            }
        }

        Matrix1::new(self.width(), self.height(), data)
    }

    /// Rotates the image 270 degrees clockwise (90 degrees counter-clockwise).
    fn rotate_270(&self) -> Self {
        let new_width = self.height();
        let new_height = self.width();
        let mut data = vec![0u8; new_width * new_height];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = y * self.width() + x;
                let new_x = y;
                let new_y = self.width() - 1 - x;
                let dst_idx = new_y * new_width + new_x;
                data[dst_idx] = self.data()[src_idx];
            }
        }

        Matrix1::new(new_width, new_height, data)
    }

    /// Rotates the image by an arbitrary angle using interpolation.
    ///
    /// This method supports any rotation angle (not just 90-degree increments).
    /// The output image is sized to contain the entire rotated image without cropping.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle (use `Rotation::Degrees()` or `Rotation::Radians()`)
    /// * `method` - Interpolation method for sampling rotated pixels
    ///
    /// # Returns
    ///
    /// A new Matrix1 with the rotated image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix1, Rotation, InterpolationMethod};
    ///
    /// let image = Matrix1::zeros(640, 480);
    ///
    /// // Rotate 45 degrees clockwise
    /// let rotated = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
    ///
    /// // Rotate PI/4 radians
    /// let rotated = image.rotate_custom(Rotation::Radians(std::f32::consts::PI / 4.0), InterpolationMethod::Bilinear);
    /// ```
    pub fn rotate_custom(&self, angle: Rotation, method: InterpolationMethod) -> Self {
        let angle_rad = angle.to_radians();
        let cos_a = cosf(angle_rad);
        let sin_a = sinf(angle_rad);

        let w = self.width() as f32;
        let h = self.height() as f32;

        // Calculate corners of rotated image
        let corners = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (x, y) in corners.iter() {
            let rx = x * cos_a - y * sin_a;
            let ry = x * sin_a + y * cos_a;
            min_x = if rx < min_x { rx } else { min_x };
            max_x = if rx > max_x { rx } else { max_x };
            min_y = if ry < min_y { ry } else { min_y };
            max_y = if ry > max_y { ry } else { max_y };
        }

        let new_width = ceilf(max_x - min_x) as usize;
        let new_height = ceilf(max_y - min_y) as usize;
        let mut data = vec![0u8; new_width * new_height];

        let center_x = w / 2.0;
        let center_y = h / 2.0;
        let new_center_x = new_width as f32 / 2.0;
        let new_center_y = new_height as f32 / 2.0;

        for y in 0..new_height {
            for x in 0..new_width {
                // Translate to origin
                let dx = x as f32 - new_center_x;
                let dy = y as f32 - new_center_y;

                // Inverse rotation
                let src_x = dx * cos_a + dy * sin_a + center_x;
                let src_y = -dx * sin_a + dy * cos_a + center_y;

                // Sample pixel based on interpolation method
                let value = match method {
                    InterpolationMethod::NearestNeighbor => self.sample_nearest(src_x, src_y),
                    InterpolationMethod::Bilinear => self.sample_bilinear(src_x, src_y),
                };

                data[y * new_width + x] = value;
            }
        }

        Matrix1::new(new_width, new_height, data)
    }

    /// Sample pixel using nearest neighbor interpolation.
    fn sample_nearest(&self, x: f32, y: f32) -> u8 {
        let ix = roundf(x) as isize;
        let iy = roundf(y) as isize;

        if ix < 0 || iy < 0 || ix >= self.width() as isize || iy >= self.height() as isize {
            return 0; // Out of bounds
        }

        self.data()[(iy as usize) * self.width() + (ix as usize)]
    }

    /// Sample pixel using bilinear interpolation.
    fn sample_bilinear(&self, x: f32, y: f32) -> u8 {
        if x < 0.0 || y < 0.0 || x >= self.width() as f32 || y >= self.height() as f32 {
            return 0; // Out of bounds
        }

        let x1 = floorf(x) as usize;
        let y1 = floorf(y) as usize;
        let x2 = (x1 + 1).min(self.width() - 1);
        let y2 = (y1 + 1).min(self.height() - 1);

        let dx = x - x1 as f32;
        let dy = y - y1 as f32;

        let p11 = self.data()[y1 * self.width() + x1] as f32;
        let p12 = self.data()[y2 * self.width() + x1] as f32;
        let p21 = self.data()[y1 * self.width() + x2] as f32;
        let p22 = self.data()[y2 * self.width() + x2] as f32;

        let val = p11 * (1.0 - dx) * (1.0 - dy)
            + p21 * dx * (1.0 - dy)
            + p12 * (1.0 - dx) * dy
            + p22 * dx * dy;

        roundf(val) as u8
    }
}

impl Matrix3 {
    /// Resizes the image to the specified dimensions.
    ///
    /// # Arguments
    ///
    /// * `new_width` - Target width
    /// * `new_height` - Target height
    /// * `method` - Interpolation method to use
    ///
    /// # Returns
    ///
    /// A new Matrix3 with the resized image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix3, InterpolationMethod};
    ///
    /// let image = Matrix3::zeros(640, 480);
    /// let resized = image.resize(320, 240, InterpolationMethod::Bilinear);
    /// assert_eq!(resized.width(), 320);
    /// assert_eq!(resized.height(), 240);
    /// ```
    pub fn resize(&self, new_width: usize, new_height: usize, method: InterpolationMethod) -> Self {
        match method {
            InterpolationMethod::NearestNeighbor => self.resize_nearest(new_width, new_height),
            InterpolationMethod::Bilinear => self.resize_bilinear(new_width, new_height),
        }
    }

    /// Resizes using nearest neighbor interpolation.
    fn resize_nearest(&self, new_width: usize, new_height: usize) -> Self {
        let mut data = vec![0u8; new_width * new_height * 3];

        let x_ratio = self.width() as f32 / new_width as f32;
        let y_ratio = self.height() as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio) as usize;
                let src_y = (y as f32 * y_ratio) as usize;

                let src_x = src_x.min(self.width() - 1);
                let src_y = src_y.min(self.height() - 1);

                let src_idx = (src_y * self.width() + src_x) * 3;
                let dst_idx = (y * new_width + x) * 3;

                data[dst_idx] = self.data()[src_idx];
                data[dst_idx + 1] = self.data()[src_idx + 1];
                data[dst_idx + 2] = self.data()[src_idx + 2];
            }
        }

        Matrix3::new(new_width, new_height, data)
    }

    /// Resizes using bilinear interpolation.
    fn resize_bilinear(&self, new_width: usize, new_height: usize) -> Self {
        let mut data = vec![0u8; new_width * new_height * 3];

        let x_ratio = (self.width() - 1) as f32 / new_width as f32;
        let y_ratio = (self.height() - 1) as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x as f32 * x_ratio;
                let src_y = y as f32 * y_ratio;

                let x1 = floorf(src_x) as usize;
                let y1 = floorf(src_y) as usize;
                let x2 = (x1 + 1).min(self.width() - 1);
                let y2 = (y1 + 1).min(self.height() - 1);

                let dx = src_x - x1 as f32;
                let dy = src_y - y1 as f32;

                let dst_idx = (y * new_width + x) * 3;

                // Interpolate each channel
                for c in 0..3 {
                    let p11 = self.data()[(y1 * self.width() + x1) * 3 + c] as f32;
                    let p12 = self.data()[(y2 * self.width() + x1) * 3 + c] as f32;
                    let p21 = self.data()[(y1 * self.width() + x2) * 3 + c] as f32;
                    let p22 = self.data()[(y2 * self.width() + x2) * 3 + c] as f32;

                    let val = p11 * (1.0 - dx) * (1.0 - dy)
                        + p21 * dx * (1.0 - dy)
                        + p12 * (1.0 - dx) * dy
                        + p22 * dx * dy;

                    data[dst_idx + c] = roundf(val) as u8;
                }
            }
        }

        Matrix3::new(new_width, new_height, data)
    }

    /// Crops the image to the specified rectangle.
    ///
    /// # Arguments
    ///
    /// * `x` - X-coordinate of the top-left corner
    /// * `y` - Y-coordinate of the top-left corner
    /// * `width` - Width of the crop region
    /// * `height` - Height of the crop region
    ///
    /// # Returns
    ///
    /// Returns Some(Matrix3) if the crop region is valid, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::Matrix3;
    ///
    /// let image = Matrix3::zeros(640, 480);
    /// let cropped = image.crop(100, 100, 200, 200).unwrap();
    /// assert_eq!(cropped.width(), 200);
    /// assert_eq!(cropped.height(), 200);
    /// ```
    pub fn crop(&self, x: usize, y: usize, width: usize, height: usize) -> Option<Self> {
        // Validate crop region
        if x + width > self.width() || y + height > self.height() {
            return None;
        }

        let mut data = vec![0u8; width * height * 3];

        for row in 0..height {
            let src_start = ((y + row) * self.width() + x) * 3;
            let dst_start = row * width * 3;
            let len = width * 3;
            data[dst_start..dst_start + len]
                .copy_from_slice(&self.data()[src_start..src_start + len]);
        }

        Some(Matrix3::new(width, height, data))
    }

    /// Rotates the image by the specified angle.
    ///
    /// Only 90-degree rotations are supported for efficiency and lossless transformation.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle (90, 180, or 270 degrees)
    ///
    /// # Returns
    ///
    /// A new Matrix3 with the rotated image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix3, RotationAngle};
    ///
    /// let image = Matrix3::zeros(640, 480);
    /// let rotated = image.rotate(RotationAngle::Rotate90);
    /// assert_eq!(rotated.width(), 480);
    /// assert_eq!(rotated.height(), 640);
    /// ```
    pub fn rotate(&self, angle: RotationAngle) -> Self {
        match angle {
            RotationAngle::Rotate90 => self.rotate_90(),
            RotationAngle::Rotate180 => self.rotate_180(),
            RotationAngle::Rotate270 => self.rotate_270(),
        }
    }

    /// Rotates the image 90 degrees clockwise.
    fn rotate_90(&self) -> Self {
        let new_width = self.height();
        let new_height = self.width();
        let mut data = vec![0u8; new_width * new_height * 3];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = (y * self.width() + x) * 3;
                let new_x = self.height() - 1 - y;
                let new_y = x;
                let dst_idx = (new_y * new_width + new_x) * 3;

                data[dst_idx] = self.data()[src_idx];
                data[dst_idx + 1] = self.data()[src_idx + 1];
                data[dst_idx + 2] = self.data()[src_idx + 2];
            }
        }

        Matrix3::new(new_width, new_height, data)
    }

    /// Rotates the image 180 degrees.
    fn rotate_180(&self) -> Self {
        let mut data = vec![0u8; self.width() * self.height() * 3];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = (y * self.width() + x) * 3;
                let new_x = self.width() - 1 - x;
                let new_y = self.height() - 1 - y;
                let dst_idx = (new_y * self.width() + new_x) * 3;

                data[dst_idx] = self.data()[src_idx];
                data[dst_idx + 1] = self.data()[src_idx + 1];
                data[dst_idx + 2] = self.data()[src_idx + 2];
            }
        }

        Matrix3::new(self.width(), self.height(), data)
    }

    /// Rotates the image 270 degrees clockwise (90 degrees counter-clockwise).
    fn rotate_270(&self) -> Self {
        let new_width = self.height();
        let new_height = self.width();
        let mut data = vec![0u8; new_width * new_height * 3];

        for y in 0..self.height() {
            for x in 0..self.width() {
                let src_idx = (y * self.width() + x) * 3;
                let new_x = y;
                let new_y = self.width() - 1 - x;
                let dst_idx = (new_y * new_width + new_x) * 3;

                data[dst_idx] = self.data()[src_idx];
                data[dst_idx + 1] = self.data()[src_idx + 1];
                data[dst_idx + 2] = self.data()[src_idx + 2];
            }
        }

        Matrix3::new(new_width, new_height, data)
    }

    /// Rotates the image by an arbitrary angle using interpolation.
    ///
    /// This method supports any rotation angle (not just 90-degree increments).
    /// The output image is sized to contain the entire rotated image without cropping.
    ///
    /// # Arguments
    ///
    /// * `angle` - Rotation angle (use `Rotation::Degrees()` or `Rotation::Radians()`)
    /// * `method` - Interpolation method for sampling rotated pixels
    ///
    /// # Returns
    ///
    /// A new Matrix3 with the rotated image data.
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::{Matrix3, Rotation, InterpolationMethod};
    ///
    /// let image = Matrix3::zeros(640, 480);
    ///
    /// // Rotate 45 degrees clockwise
    /// let rotated = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
    ///
    /// // Rotate PI/4 radians
    /// let rotated = image.rotate_custom(Rotation::Radians(std::f32::consts::PI / 4.0), InterpolationMethod::Bilinear);
    /// ```
    pub fn rotate_custom(&self, angle: Rotation, method: InterpolationMethod) -> Self {
        let angle_rad = angle.to_radians();
        let cos_a = cosf(angle_rad);
        let sin_a = sinf(angle_rad);

        let w = self.width() as f32;
        let h = self.height() as f32;

        // Calculate corners of rotated image
        let corners = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for (x, y) in corners.iter() {
            let rx = x * cos_a - y * sin_a;
            let ry = x * sin_a + y * cos_a;
            min_x = if rx < min_x { rx } else { min_x };
            max_x = if rx > max_x { rx } else { max_x };
            min_y = if ry < min_y { ry } else { min_y };
            max_y = if ry > max_y { ry } else { max_y };
        }

        let new_width = ceilf(max_x - min_x) as usize;
        let new_height = ceilf(max_y - min_y) as usize;
        let mut data = vec![0u8; new_width * new_height * 3];

        let center_x = w / 2.0;
        let center_y = h / 2.0;
        let new_center_x = new_width as f32 / 2.0;
        let new_center_y = new_height as f32 / 2.0;

        for y in 0..new_height {
            for x in 0..new_width {
                // Translate to origin
                let dx = x as f32 - new_center_x;
                let dy = y as f32 - new_center_y;

                // Inverse rotation
                let src_x = dx * cos_a + dy * sin_a + center_x;
                let src_y = -dx * sin_a + dy * cos_a + center_y;

                // Sample pixel based on interpolation method
                let (r, g, b) = match method {
                    InterpolationMethod::NearestNeighbor => self.sample_nearest(src_x, src_y),
                    InterpolationMethod::Bilinear => self.sample_bilinear(src_x, src_y),
                };

                let idx = (y * new_width + x) * 3;
                data[idx] = r;
                data[idx + 1] = g;
                data[idx + 2] = b;
            }
        }

        Matrix3::new(new_width, new_height, data)
    }

    /// Sample pixel using nearest neighbor interpolation.
    fn sample_nearest(&self, x: f32, y: f32) -> (u8, u8, u8) {
        let ix = roundf(x) as isize;
        let iy = roundf(y) as isize;

        if ix < 0 || iy < 0 || ix >= self.width() as isize || iy >= self.height() as isize {
            return (0, 0, 0); // Out of bounds
        }

        let idx = ((iy as usize) * self.width() + (ix as usize)) * 3;
        let data = self.data();
        (data[idx], data[idx + 1], data[idx + 2])
    }

    /// Sample pixel using bilinear interpolation.
    fn sample_bilinear(&self, x: f32, y: f32) -> (u8, u8, u8) {
        if x < 0.0 || y < 0.0 || x >= self.width() as f32 || y >= self.height() as f32 {
            return (0, 0, 0); // Out of bounds
        }

        let x1 = floorf(x) as usize;
        let y1 = floorf(y) as usize;
        let x2 = (x1 + 1).min(self.width() - 1);
        let y2 = (y1 + 1).min(self.height() - 1);

        let dx = x - x1 as f32;
        let dy = y - y1 as f32;

        let data = self.data();
        let mut result = [0u8; 3];

        for c in 0..3 {
            let p11 = data[(y1 * self.width() + x1) * 3 + c] as f32;
            let p12 = data[(y2 * self.width() + x1) * 3 + c] as f32;
            let p21 = data[(y1 * self.width() + x2) * 3 + c] as f32;
            let p22 = data[(y2 * self.width() + x2) * 3 + c] as f32;

            let val = p11 * (1.0 - dx) * (1.0 - dy)
                + p21 * dx * (1.0 - dy)
                + p12 * (1.0 - dx) * dy
                + p22 * dx * dy;

            result[c] = roundf(val) as u8;
        }

        (result[0], result[1], result[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_nearest_matrix1() {
        let mut data = vec![0u8; 4 * 4];
        for (i, pixel) in data.iter_mut().enumerate().take(16) {
            *pixel = (i * 16) as u8;
        }
        let image = Matrix1::new(4, 4, data);
        let resized = image.resize(2, 2, InterpolationMethod::NearestNeighbor);

        assert_eq!(resized.width(), 2);
        assert_eq!(resized.height(), 2);
    }

    #[test]
    fn test_resize_bilinear_matrix1() {
        let data = vec![255u8; 10 * 10];
        let image = Matrix1::new(10, 10, data);
        let resized = image.resize(5, 5, InterpolationMethod::Bilinear);

        assert_eq!(resized.width(), 5);
        assert_eq!(resized.height(), 5);
        // All pixels should still be 255
        assert!(resized.data().iter().all(|&x| x == 255));
    }

    #[test]
    fn test_crop_matrix1() {
        let mut data = vec![0u8; 10 * 10];
        for (i, pixel) in data.iter_mut().enumerate().take(100) {
            *pixel = (i % 256) as u8;
        }
        let image = Matrix1::new(10, 10, data);
        let cropped = image.crop(2, 2, 5, 5).unwrap();

        assert_eq!(cropped.width(), 5);
        assert_eq!(cropped.height(), 5);
    }

    #[test]
    fn test_crop_invalid_matrix1() {
        let image = Matrix1::zeros(10, 10);
        let cropped = image.crop(8, 8, 5, 5);
        assert!(cropped.is_none());
    }

    #[test]
    fn test_rotate_90_matrix1() {
        let mut data = vec![0u8; 3 * 2];
        data[0] = 1;
        data[1] = 2;
        data[2] = 3;
        data[3] = 4;
        data[4] = 5;
        data[5] = 6;

        let image = Matrix1::new(3, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate90);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 3);
    }

    #[test]
    fn test_rotate_180_matrix1() {
        let data = vec![1, 2, 3, 4];
        let image = Matrix1::new(2, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate180);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 2);
        assert_eq!(rotated.data()[0], 4);
        assert_eq!(rotated.data()[3], 1);
    }

    #[test]
    fn test_rotate_270_matrix1() {
        let mut data = vec![0u8; 3 * 2];
        for (i, pixel) in data.iter_mut().enumerate().take(6) {
            *pixel = (i + 1) as u8;
        }

        let image = Matrix1::new(3, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate270);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 3);
    }

    #[test]
    fn test_resize_nearest_matrix3() {
        let mut data = vec![0u8; 4 * 4 * 3];
        for (i, pixel) in data.iter_mut().enumerate().take(4 * 4 * 3) {
            *pixel = (i % 256) as u8;
        }
        let image = Matrix3::new(4, 4, data);
        let resized = image.resize(2, 2, InterpolationMethod::NearestNeighbor);

        assert_eq!(resized.width(), 2);
        assert_eq!(resized.height(), 2);
    }

    #[test]
    fn test_resize_bilinear_matrix3() {
        let data = vec![128u8; 10 * 10 * 3];
        let image = Matrix3::new(10, 10, data);
        let resized = image.resize(5, 5, InterpolationMethod::Bilinear);

        assert_eq!(resized.width(), 5);
        assert_eq!(resized.height(), 5);
        // All pixels should still be 128
        assert!(resized.data().iter().all(|&x| x == 128));
    }

    #[test]
    fn test_crop_matrix3() {
        let mut data = vec![0u8; 10 * 10 * 3];
        for (i, pixel) in data.iter_mut().enumerate().take(10 * 10 * 3) {
            *pixel = (i % 256) as u8;
        }
        let image = Matrix3::new(10, 10, data);
        let cropped = image.crop(2, 2, 5, 5).unwrap();

        assert_eq!(cropped.width(), 5);
        assert_eq!(cropped.height(), 5);
    }

    #[test]
    fn test_crop_invalid_matrix3() {
        let image = Matrix3::zeros(10, 10);
        let cropped = image.crop(8, 8, 5, 5);
        assert!(cropped.is_none());
    }

    #[test]
    fn test_rotate_90_matrix3() {
        let mut data = vec![0u8; 3 * 2 * 3];
        for (i, pixel) in data.iter_mut().enumerate().take(18) {
            *pixel = (i % 256) as u8;
        }

        let image = Matrix3::new(3, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate90);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 3);
    }

    #[test]
    fn test_rotate_180_matrix3() {
        let mut data = vec![0u8; 2 * 2 * 3];
        for (i, pixel) in data.iter_mut().enumerate().take(12) {
            *pixel = (i + 1) as u8;
        }
        let image = Matrix3::new(2, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate180);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 2);
    }

    #[test]
    fn test_rotate_270_matrix3() {
        let mut data = vec![0u8; 3 * 2 * 3];
        for (i, pixel) in data.iter_mut().enumerate().take(18) {
            *pixel = (i % 256) as u8;
        }

        let image = Matrix3::new(3, 2, data);
        let rotated = image.rotate(RotationAngle::Rotate270);

        assert_eq!(rotated.width(), 2);
        assert_eq!(rotated.height(), 3);
    }

    #[test]
    fn test_rotate_custom_degrees_matrix1() {
        let image = Matrix1::zeros(100, 100);

        // Test 0 degrees (should keep same dimensions approximately)
        let rotated = image.rotate_custom(Rotation::Degrees(0.0), InterpolationMethod::Bilinear);
        assert_eq!(rotated.width(), 100);
        assert_eq!(rotated.height(), 100);

        // Test 45 degrees (should increase dimensions)
        let rotated = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
        assert!(rotated.width() > 100);
        assert!(rotated.height() > 100);

        // Test 90 degrees (should swap dimensions approximately)
        let rotated = image.rotate_custom(
            Rotation::Degrees(90.0),
            InterpolationMethod::NearestNeighbor,
        );
        assert!(rotated.width() >= 100 && rotated.width() <= 101);
        assert!(rotated.height() >= 100 && rotated.height() <= 101);
    }

    #[test]
    fn test_rotate_custom_radians_matrix1() {
        let image = Matrix1::zeros(100, 100);

        // Test PI/4 radians (45 degrees)
        let rotated =
            image.rotate_custom(Rotation::Radians(PI / 4.0), InterpolationMethod::Bilinear);
        assert!(rotated.width() > 100);
        assert!(rotated.height() > 100);

        // Test PI/2 radians (90 degrees)
        let rotated =
            image.rotate_custom(Rotation::Radians(PI / 2.0), InterpolationMethod::Bilinear);
        assert!(rotated.width() >= 100 && rotated.width() <= 101);
    }

    #[test]
    fn test_rotate_custom_degrees_matrix3() {
        let image = Matrix3::zeros(100, 100);

        // Test 0 degrees
        let rotated = image.rotate_custom(Rotation::Degrees(0.0), InterpolationMethod::Bilinear);
        assert_eq!(rotated.width(), 100);
        assert_eq!(rotated.height(), 100);

        // Test 45 degrees
        let rotated = image.rotate_custom(Rotation::Degrees(45.0), InterpolationMethod::Bilinear);
        assert!(rotated.width() > 100);
        assert!(rotated.height() > 100);

        // Test 180 degrees (should keep same dimensions)
        let rotated = image.rotate_custom(
            Rotation::Degrees(180.0),
            InterpolationMethod::NearestNeighbor,
        );
        assert!(rotated.width() >= 100 && rotated.width() <= 101);
        assert!(rotated.height() >= 100 && rotated.height() <= 101);
    }

    #[test]
    fn test_rotate_custom_radians_matrix3() {
        let image = Matrix3::zeros(100, 100);

        // Test PI/6 radians (30 degrees)
        let rotated =
            image.rotate_custom(Rotation::Radians(PI / 6.0), InterpolationMethod::Bilinear);
        assert!(rotated.width() > 100);
        assert!(rotated.height() > 100);

        // Test PI radians (180 degrees)
        let rotated = image.rotate_custom(Rotation::Radians(PI), InterpolationMethod::Bilinear);
        assert!(rotated.width() >= 100 && rotated.width() <= 101);
    }

    #[test]
    fn test_rotation_conversion() {
        let deg_45 = Rotation::Degrees(45.0);
        let rad_pi4 = Rotation::Radians(PI / 4.0);

        // Test conversion to radians
        let deg_to_rad = deg_45.to_radians();
        let rad_same = rad_pi4.to_radians();
        assert!((deg_to_rad - rad_same).abs() < 0.001);

        // Test conversion to degrees
        let deg_same = deg_45.to_degrees();
        let rad_to_deg = rad_pi4.to_degrees();
        assert!((deg_same - rad_to_deg).abs() < 0.001);
    }

    #[test]
    fn test_rotate_custom_preserves_data() {
        // Create a larger 10x10 image with a bright center region
        let mut data = vec![0u8; 10 * 10];
        // Set a 3x3 bright region in the center
        for y in 4..7 {
            for x in 4..7 {
                data[y * 10 + x] = 255;
            }
        }
        let image = Matrix1::new(10, 10, data);

        // Rotate by a small angle and check that bright region is preserved
        let rotated = image.rotate_custom(Rotation::Degrees(10.0), InterpolationMethod::Bilinear);

        // Find the brightest pixel in rotated image
        let max_val = rotated.data().iter().max().unwrap();
        assert!(*max_val > 200); // Should still have bright pixels
    }
}
