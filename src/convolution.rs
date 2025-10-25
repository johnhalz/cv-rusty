//! Convolution operations for image processing.
//!
//! This module provides efficient 2D convolution operations with support for
//! parallel processing when the `parallel` feature is enabled.

#[cfg(not(feature = "std"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::matrix::{Matrix1, Matrix3};

/// Boundary handling method for convolution operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderMode {
    /// Pad with zeros outside the image boundary
    Zero,
    /// Replicate the edge pixels
    Replicate,
    /// Reflect across the edge (abcd|dcba)
    Reflect,
    /// Wrap around to the opposite edge
    Wrap,
}

/// A 2D convolution kernel.
#[derive(Debug, Clone)]
pub struct Kernel {
    /// Kernel width (must be odd)
    width: usize,
    /// Kernel height (must be odd)
    height: usize,
    /// Kernel data in row-major order
    data: Vec<f32>,
}

impl Kernel {
    /// Creates a new convolution kernel.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the kernel (must be odd)
    /// * `height` - Height of the kernel (must be odd)
    /// * `data` - Kernel weights in row-major order
    ///
    /// # Panics
    ///
    /// Panics if width or height is even, or if data length doesn't match width * height.
    pub fn new(width: usize, height: usize, data: Vec<f32>) -> Self {
        assert!(width % 2 == 1, "Kernel width must be odd");
        assert!(height % 2 == 1, "Kernel height must be odd");
        assert_eq!(
            data.len(),
            width * height,
            "Data length must match width * height"
        );
        Self {
            width,
            height,
            data,
        }
    }

    /// Returns the width of the kernel.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the kernel.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the kernel data.
    pub fn data(&self) -> &[f32] {
        &self.data
    }

    /// Creates a box blur kernel (uniform averaging).
    ///
    /// # Arguments
    ///
    /// * `size` - Size of the kernel (must be odd)
    pub fn box_blur(size: usize) -> Self {
        assert!(size % 2 == 1, "Kernel size must be odd");
        let count = size * size;
        let value = 1.0 / count as f32;
        Self {
            width: size,
            height: size,
            data: vec![value; count],
        }
    }

    /// Creates a Gaussian blur kernel.
    ///
    /// # Arguments
    ///
    /// * `size` - Size of the kernel (must be odd)
    /// * `sigma` - Standard deviation of the Gaussian
    pub fn gaussian(size: usize, sigma: f32) -> Self {
        assert!(size % 2 == 1, "Kernel size must be odd");
        let half = (size / 2) as i32;
        let mut data = Vec::with_capacity(size * size);
        let mut sum = 0.0;

        for y in -half..=half {
            for x in -half..=half {
                let value = gaussian_2d(x as f32, y as f32, sigma);
                data.push(value);
                sum += value;
            }
        }

        // Normalize
        for value in &mut data {
            *value /= sum;
        }

        Self {
            width: size,
            height: size,
            data,
        }
    }

    /// Creates a Sobel X kernel (horizontal edge detection).
    pub fn sobel_x() -> Self {
        Self::new(3, 3, vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0])
    }

    /// Creates a Sobel Y kernel (vertical edge detection).
    pub fn sobel_y() -> Self {
        Self::new(3, 3, vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0])
    }

    /// Creates a Laplacian kernel (edge detection).
    pub fn laplacian() -> Self {
        Self::new(3, 3, vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0])
    }

    /// Creates a sharpening kernel.
    pub fn sharpen() -> Self {
        Self::new(3, 3, vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0])
    }
}

/// 2D Gaussian function
fn gaussian_2d(x: f32, y: f32, sigma: f32) -> f32 {
    let coefficient = 1.0 / (2.0 * core::f32::consts::PI * sigma * sigma);
    let exponent = -(x * x + y * y) / (2.0 * sigma * sigma);
    #[cfg(feature = "std")]
    {
        coefficient * exponent.exp()
    }
    #[cfg(not(feature = "std"))]
    {
        coefficient * libm::expf(exponent)
    }
}

impl Matrix1 {
    /// Applies a convolution kernel to the grayscale image.
    ///
    /// # Arguments
    ///
    /// * `kernel` - The convolution kernel to apply
    /// * `border_mode` - How to handle borders
    ///
    /// # Returns
    ///
    /// A new Matrix1 with the convolution applied.
    pub fn convolve(&self, kernel: &Kernel, border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();

        let k_half_w = (kernel.width() / 2) as i32;
        let k_half_h = (kernel.height() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width];
                    for (x, pixel) in row.iter_mut().enumerate() {
                        *pixel = self.convolve_pixel(
                            x as i32,
                            y as i32,
                            kernel,
                            k_half_w,
                            k_half_h,
                            border_mode,
                        );
                    }
                    row
                })
                .collect();
            Matrix1::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix1::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let value = self.convolve_pixel(
                        x as i32,
                        y as i32,
                        kernel,
                        k_half_w,
                        k_half_h,
                        border_mode,
                    );
                    result.set_pixel(x, y, value);
                }
            }
            result
        }
    }

    /// Convolves a single pixel.
    #[inline]
    fn convolve_pixel(
        &self,
        x: i32,
        y: i32,
        kernel: &Kernel,
        k_half_w: i32,
        k_half_h: i32,
        border_mode: BorderMode,
    ) -> u8 {
        let mut sum = 0.0f32;

        for ky in 0..kernel.height() as i32 {
            for kx in 0..kernel.width() as i32 {
                let img_x = x + kx - k_half_w;
                let img_y = y + ky - k_half_h;

                let pixel_value = self.get_pixel_with_border(img_x, img_y, border_mode);
                let kernel_value = kernel.data()[(ky * kernel.width() as i32 + kx) as usize];

                sum += pixel_value as f32 * kernel_value;
            }
        }

        // Clamp to valid u8 range
        sum.clamp(0.0, 255.0) as u8
    }

    /// Gets a pixel value with border handling.
    #[inline]
    fn get_pixel_with_border(&self, x: i32, y: i32, border_mode: BorderMode) -> u8 {
        let width = self.width() as i32;
        let height = self.height() as i32;

        let (x, y) = match border_mode {
            BorderMode::Zero => {
                if x < 0 || x >= width || y < 0 || y >= height {
                    return 0;
                }
                (x as usize, y as usize)
            }
            BorderMode::Replicate => {
                let x = x.max(0).min(width - 1) as usize;
                let y = y.max(0).min(height - 1) as usize;
                (x, y)
            }
            BorderMode::Reflect => {
                let x = reflect_coordinate(x, width) as usize;
                let y = reflect_coordinate(y, height) as usize;
                (x, y)
            }
            BorderMode::Wrap => {
                let x = wrap_coordinate(x, width) as usize;
                let y = wrap_coordinate(y, height) as usize;
                (x, y)
            }
        };

        self.get_pixel(x, y).unwrap_or(0)
    }

    /// Applies a separable convolution (more efficient for separable kernels).
    ///
    /// # Arguments
    ///
    /// * `kernel_x` - Horizontal 1D kernel
    /// * `kernel_y` - Vertical 1D kernel
    /// * `border_mode` - How to handle borders
    ///
    /// # Returns
    ///
    /// A new Matrix1 with the convolution applied.
    pub fn convolve_separable(
        &self,
        kernel_x: &[f32],
        kernel_y: &[f32],
        border_mode: BorderMode,
    ) -> Self {
        assert!(kernel_x.len() % 2 == 1, "Kernel length must be odd");
        assert!(kernel_y.len() % 2 == 1, "Kernel length must be odd");

        // First pass: horizontal
        let temp = self.convolve_horizontal(kernel_x, border_mode);

        // Second pass: vertical
        temp.convolve_vertical(kernel_y, border_mode)
    }

    /// Applies horizontal 1D convolution.
    fn convolve_horizontal(&self, kernel: &[f32], border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();
        let k_half = (kernel.len() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width];
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let mut sum = 0.0f32;
                        for k in 0..kernel.len() as i32 {
                            let img_x = x as i32 + k - k_half;
                            let pixel_value =
                                self.get_pixel_with_border(img_x, y as i32, border_mode);
                            sum += pixel_value as f32 * kernel[k as usize];
                        }
                        *pixel = sum.clamp(0.0, 255.0) as u8;
                    }
                    row
                })
                .collect();
            Matrix1::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix1::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let mut sum = 0.0f32;
                    for k in 0..kernel.len() as i32 {
                        let img_x = x as i32 + k - k_half;
                        let pixel_value = self.get_pixel_with_border(img_x, y as i32, border_mode);
                        sum += pixel_value as f32 * kernel[k as usize];
                    }
                    result.set_pixel(x, y, sum.max(0.0).min(255.0) as u8);
                }
            }
            result
        }
    }

    /// Applies vertical 1D convolution.
    fn convolve_vertical(&self, kernel: &[f32], border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();
        let k_half = (kernel.len() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width];
                    for (x, pixel) in row.iter_mut().enumerate() {
                        let mut sum = 0.0f32;
                        for k in 0..kernel.len() as i32 {
                            let img_y = y as i32 + k - k_half;
                            let pixel_value =
                                self.get_pixel_with_border(x as i32, img_y, border_mode);
                            sum += pixel_value as f32 * kernel[k as usize];
                        }
                        *pixel = sum.clamp(0.0, 255.0) as u8;
                    }
                    row
                })
                .collect();
            Matrix1::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix1::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let mut sum = 0.0f32;
                    for k in 0..kernel.len() as i32 {
                        let img_y = y as i32 + k - k_half;
                        let pixel_value = self.get_pixel_with_border(x as i32, img_y, border_mode);
                        sum += pixel_value as f32 * kernel[k as usize];
                    }
                    result.set_pixel(x, y, sum.max(0.0).min(255.0) as u8);
                }
            }
            result
        }
    }
}

impl Matrix3 {
    /// Applies a convolution kernel to the RGB image.
    ///
    /// The kernel is applied independently to each channel.
    ///
    /// # Arguments
    ///
    /// * `kernel` - The convolution kernel to apply
    /// * `border_mode` - How to handle borders
    ///
    /// # Returns
    ///
    /// A new Matrix3 with the convolution applied.
    pub fn convolve(&self, kernel: &Kernel, border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();

        let k_half_w = (kernel.width() / 2) as i32;
        let k_half_h = (kernel.height() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width * 3];
                    for x in 0..width {
                        let (r, g, b) = self.convolve_pixel(
                            x as i32,
                            y as i32,
                            kernel,
                            k_half_w,
                            k_half_h,
                            border_mode,
                        );
                        row[x * 3] = r;
                        row[x * 3 + 1] = g;
                        row[x * 3 + 2] = b;
                    }
                    row
                })
                .collect();
            Matrix3::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix3::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let (r, g, b) = self.convolve_pixel(
                        x as i32,
                        y as i32,
                        kernel,
                        k_half_w,
                        k_half_h,
                        border_mode,
                    );
                    result.set_pixel(x, y, r, g, b);
                }
            }
            result
        }
    }

    /// Convolves a single pixel across all channels.
    #[inline]
    fn convolve_pixel(
        &self,
        x: i32,
        y: i32,
        kernel: &Kernel,
        k_half_w: i32,
        k_half_h: i32,
        border_mode: BorderMode,
    ) -> (u8, u8, u8) {
        let mut sum_r = 0.0f32;
        let mut sum_g = 0.0f32;
        let mut sum_b = 0.0f32;

        for ky in 0..kernel.height() as i32 {
            for kx in 0..kernel.width() as i32 {
                let img_x = x + kx - k_half_w;
                let img_y = y + ky - k_half_h;

                let (r, g, b) = self.get_pixel_with_border(img_x, img_y, border_mode);
                let kernel_value = kernel.data()[(ky * kernel.width() as i32 + kx) as usize];

                sum_r += r as f32 * kernel_value;
                sum_g += g as f32 * kernel_value;
                sum_b += b as f32 * kernel_value;
            }
        }

        // Clamp to valid u8 range
        (
            sum_r.clamp(0.0, 255.0) as u8,
            sum_g.clamp(0.0, 255.0) as u8,
            sum_b.clamp(0.0, 255.0) as u8,
        )
    }

    /// Gets a pixel value with border handling.
    #[inline]
    fn get_pixel_with_border(&self, x: i32, y: i32, border_mode: BorderMode) -> (u8, u8, u8) {
        let width = self.width() as i32;
        let height = self.height() as i32;

        let (x, y) = match border_mode {
            BorderMode::Zero => {
                if x < 0 || x >= width || y < 0 || y >= height {
                    return (0, 0, 0);
                }
                (x as usize, y as usize)
            }
            BorderMode::Replicate => {
                let x = x.max(0).min(width - 1) as usize;
                let y = y.max(0).min(height - 1) as usize;
                (x, y)
            }
            BorderMode::Reflect => {
                let x = reflect_coordinate(x, width) as usize;
                let y = reflect_coordinate(y, height) as usize;
                (x, y)
            }
            BorderMode::Wrap => {
                let x = wrap_coordinate(x, width) as usize;
                let y = wrap_coordinate(y, height) as usize;
                (x, y)
            }
        };

        self.get_pixel(x, y).unwrap_or((0, 0, 0))
    }

    /// Applies a separable convolution (more efficient for separable kernels).
    ///
    /// # Arguments
    ///
    /// * `kernel_x` - Horizontal 1D kernel
    /// * `kernel_y` - Vertical 1D kernel
    /// * `border_mode` - How to handle borders
    ///
    /// # Returns
    ///
    /// A new Matrix3 with the convolution applied.
    pub fn convolve_separable(
        &self,
        kernel_x: &[f32],
        kernel_y: &[f32],
        border_mode: BorderMode,
    ) -> Self {
        assert!(kernel_x.len() % 2 == 1, "Kernel length must be odd");
        assert!(kernel_y.len() % 2 == 1, "Kernel length must be odd");

        // First pass: horizontal
        let temp = self.convolve_horizontal(kernel_x, border_mode);

        // Second pass: vertical
        temp.convolve_vertical(kernel_y, border_mode)
    }

    /// Applies horizontal 1D convolution.
    fn convolve_horizontal(&self, kernel: &[f32], border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();
        let k_half = (kernel.len() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width * 3];
                    for x in 0..width {
                        let mut sum_r = 0.0f32;
                        let mut sum_g = 0.0f32;
                        let mut sum_b = 0.0f32;
                        for k in 0..kernel.len() as i32 {
                            let img_x = x as i32 + k - k_half;
                            let (r, g, b) =
                                self.get_pixel_with_border(img_x, y as i32, border_mode);
                            let kval = kernel[k as usize];
                            sum_r += r as f32 * kval;
                            sum_g += g as f32 * kval;
                            sum_b += b as f32 * kval;
                        }
                        row[x * 3] = sum_r.clamp(0.0, 255.0) as u8;
                        row[x * 3 + 1] = sum_g.clamp(0.0, 255.0) as u8;
                        row[x * 3 + 2] = sum_b.clamp(0.0, 255.0) as u8;
                    }
                    row
                })
                .collect();
            Matrix3::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix3::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let mut sum_r = 0.0f32;
                    let mut sum_g = 0.0f32;
                    let mut sum_b = 0.0f32;
                    for k in 0..kernel.len() as i32 {
                        let img_x = x as i32 + k - k_half;
                        let (r, g, b) = self.get_pixel_with_border(img_x, y as i32, border_mode);
                        let kval = kernel[k as usize];
                        sum_r += r as f32 * kval;
                        sum_g += g as f32 * kval;
                        sum_b += b as f32 * kval;
                    }
                    result.set_pixel(
                        x,
                        y,
                        sum_r.max(0.0).min(255.0) as u8,
                        sum_g.max(0.0).min(255.0) as u8,
                        sum_b.max(0.0).min(255.0) as u8,
                    );
                }
            }
            result
        }
    }

    /// Applies vertical 1D convolution.
    fn convolve_vertical(&self, kernel: &[f32], border_mode: BorderMode) -> Self {
        let width = self.width();
        let height = self.height();
        let k_half = (kernel.len() / 2) as i32;

        #[cfg(feature = "parallel")]
        {
            let result_data: Vec<u8> = (0..height)
                .into_par_iter()
                .flat_map(|y| {
                    let mut row = vec![0u8; width * 3];
                    for x in 0..width {
                        let mut sum_r = 0.0f32;
                        let mut sum_g = 0.0f32;
                        let mut sum_b = 0.0f32;
                        for k in 0..kernel.len() as i32 {
                            let img_y = y as i32 + k - k_half;
                            let (r, g, b) =
                                self.get_pixel_with_border(x as i32, img_y, border_mode);
                            let kval = kernel[k as usize];
                            sum_r += r as f32 * kval;
                            sum_g += g as f32 * kval;
                            sum_b += b as f32 * kval;
                        }
                        row[x * 3] = sum_r.clamp(0.0, 255.0) as u8;
                        row[x * 3 + 1] = sum_g.clamp(0.0, 255.0) as u8;
                        row[x * 3 + 2] = sum_b.clamp(0.0, 255.0) as u8;
                    }
                    row
                })
                .collect();
            Matrix3::new(width, height, result_data)
        }

        #[cfg(not(feature = "parallel"))]
        {
            let mut result = Matrix3::zeros(width, height);
            for y in 0..height {
                for x in 0..width {
                    let mut sum_r = 0.0f32;
                    let mut sum_g = 0.0f32;
                    let mut sum_b = 0.0f32;
                    for k in 0..kernel.len() as i32 {
                        let img_y = y as i32 + k - k_half;
                        let (r, g, b) = self.get_pixel_with_border(x as i32, img_y, border_mode);
                        let kval = kernel[k as usize];
                        sum_r += r as f32 * kval;
                        sum_g += g as f32 * kval;
                        sum_b += b as f32 * kval;
                    }
                    result.set_pixel(
                        x,
                        y,
                        sum_r.max(0.0).min(255.0) as u8,
                        sum_g.max(0.0).min(255.0) as u8,
                        sum_b.max(0.0).min(255.0) as u8,
                    );
                }
            }
            result
        }
    }
}

/// Reflects a coordinate around the image boundary.
#[inline]
fn reflect_coordinate(coord: i32, size: i32) -> i32 {
    let mut c = coord;
    if c < 0 {
        c = -c - 1;
    }
    if c >= size {
        c = 2 * size - c - 1;
    }
    c.max(0).min(size - 1)
}

/// Wraps a coordinate around the image boundary.
#[inline]
fn wrap_coordinate(coord: i32, size: i32) -> i32 {
    let mut c = coord % size;
    if c < 0 {
        c += size;
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_new() {
        let kernel = Kernel::new(3, 3, vec![1.0; 9]);
        assert_eq!(kernel.width(), 3);
        assert_eq!(kernel.height(), 3);
        assert_eq!(kernel.data().len(), 9);
    }

    #[test]
    #[should_panic]
    fn test_kernel_even_width() {
        Kernel::new(4, 3, vec![1.0; 12]);
    }

    #[test]
    fn test_box_blur() {
        let kernel = Kernel::box_blur(3);
        assert_eq!(kernel.width(), 3);
        assert_eq!(kernel.height(), 3);
        let expected_value = 1.0 / 9.0;
        for &value in kernel.data() {
            assert!((value - expected_value).abs() < 1e-6);
        }
    }

    #[test]
    fn test_gaussian_kernel() {
        let kernel = Kernel::gaussian(5, 1.0);
        assert_eq!(kernel.width(), 5);
        assert_eq!(kernel.height(), 5);

        // Sum should be approximately 1
        let sum: f32 = kernel.data().iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_sobel_kernels() {
        let sobel_x = Kernel::sobel_x();
        let sobel_y = Kernel::sobel_y();
        assert_eq!(sobel_x.width(), 3);
        assert_eq!(sobel_y.width(), 3);
    }

    #[test]
    fn test_matrix1_convolve_identity() {
        let mut data = vec![0u8; 10 * 10];
        data[5 * 10 + 5] = 255; // Single bright pixel in center

        let mat = Matrix1::new(10, 10, data);
        let kernel = Kernel::new(3, 3, vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);

        let result = mat.convolve(&kernel, BorderMode::Zero);
        assert_eq!(result.get_pixel(5, 5), Some(255));
    }

    #[test]
    fn test_matrix3_convolve() {
        let data = vec![128u8; 10 * 10 * 3];
        let mat = Matrix3::new(10, 10, data);
        let kernel = Kernel::box_blur(3);

        let result = mat.convolve(&kernel, BorderMode::Replicate);
        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 10);
    }

    #[test]
    fn test_border_modes() {
        let mat = Matrix1::new(5, 5, vec![100u8; 25]);

        // Test different border modes don't panic
        let kernel = Kernel::box_blur(3);
        let _ = mat.convolve(&kernel, BorderMode::Zero);
        let _ = mat.convolve(&kernel, BorderMode::Replicate);
        let _ = mat.convolve(&kernel, BorderMode::Reflect);
        let _ = mat.convolve(&kernel, BorderMode::Wrap);
    }

    #[test]
    fn test_separable_convolution() {
        let mat = Matrix1::new(10, 10, vec![128u8; 100]);
        let kernel_1d = vec![0.25, 0.5, 0.25];

        let result = mat.convolve_separable(&kernel_1d, &kernel_1d, BorderMode::Replicate);
        assert_eq!(result.width(), 10);
        assert_eq!(result.height(), 10);
    }

    #[test]
    fn test_reflect_coordinate() {
        assert_eq!(reflect_coordinate(-1, 10), 0);
        assert_eq!(reflect_coordinate(0, 10), 0);
        assert_eq!(reflect_coordinate(9, 10), 9);
        assert_eq!(reflect_coordinate(10, 10), 9);
        assert_eq!(reflect_coordinate(11, 10), 8);
    }

    #[test]
    fn test_wrap_coordinate() {
        assert_eq!(wrap_coordinate(-1, 10), 9);
        assert_eq!(wrap_coordinate(0, 10), 0);
        assert_eq!(wrap_coordinate(9, 10), 9);
        assert_eq!(wrap_coordinate(10, 10), 0);
        assert_eq!(wrap_coordinate(11, 10), 1);
    }
}
