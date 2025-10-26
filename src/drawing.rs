//! Drawing module for rendering shapes on images.
//!
//! This module is `no_std` compatible and only requires the `alloc` crate.
//!
//! # Examples
//!
//! ```
//! use cv_rusty::{Matrix3, Matrix1, draw_rectangle, draw_circle, Color};
//!
//! // Works with RGB images (Matrix3)
//! let mut rgb_image = Matrix3::zeros(640, 480);
//! draw_rectangle(
//!     &mut rgb_image,
//!     200.0, 150.0,  // position (x, y)
//!     100.0, 80.0,   // width, height
//!     0.0,           // rotation in degrees
//!     2,             // stroke width
//!     Some(Color::rgb(0, 0, 0)),       // stroke color (black)
//!     Some(Color::rgb(255, 0, 0))      // fill color (red)
//! );
//!
//! // Also works with grayscale images (Matrix1)
//! let mut gray_image = Matrix1::zeros(640, 480);
//! draw_circle(
//!     &mut gray_image,
//!     320.0, 240.0,  // position (x, y)
//!     50.0,          // radius
//!     3,             // stroke width
//!     Some(Color::gray(255)),          // stroke color (white)
//!     Some(Color::gray(100))           // fill color (dark gray)
//! );
//! ```

#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::{Matrix1, Matrix3};
use core::fmt;
use core::str::FromStr;

/// Represents a color value that can be used for both grayscale and RGB images.
///
/// # Examples
///
/// ```
/// use cv_rusty::Color;
///
/// // Create colors using constructors
/// let red = Color::rgb(255, 0, 0);
/// let gray = Color::gray(128);
///
/// // Create colors from hex strings
/// let blue = Color::from_hex("#0000FF").unwrap();
/// let green = Color::from_hex("00FF00").unwrap();
/// let cyan = Color::from_hex("#0FF").unwrap();  // 3-digit format
///
/// // Using FromStr trait
/// let yellow: Color = "#FFFF00".parse().unwrap();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Grayscale color (single channel)
    Gray(u8),
    /// RGB color (three channels)
    Rgb(u8, u8, u8),
}

impl Color {
    /// Creates a new grayscale color.
    pub fn gray(value: u8) -> Self {
        Color::Gray(value)
    }

    /// Creates a new RGB color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Creates a black color (grayscale or RGB).
    pub fn black() -> Self {
        Color::Rgb(0, 0, 0)
    }

    /// Creates a white color (grayscale or RGB).
    pub fn white() -> Self {
        Color::Rgb(255, 255, 255)
    }

    /// Converts the color to grayscale if it's RGB.
    pub fn to_gray(&self) -> u8 {
        match self {
            Color::Gray(v) => *v,
            Color::Rgb(r, g, b) => {
                // Using standard luminance formula
                ((0.299 * (*r as f32)) + (0.587 * (*g as f32)) + (0.114 * (*b as f32))) as u8
            }
        }
    }

    /// Gets RGB values, converting from grayscale if necessary.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Gray(v) => (*v, *v, *v),
            Color::Rgb(r, g, b) => (*r, *g, *b),
        }
    }

    /// Creates a color from a hex string.
    ///
    /// Supports multiple formats:
    /// - `"#RRGGBB"` - 6-digit hex with hash
    /// - `"RRGGBB"` - 6-digit hex without hash
    /// - `"#RGB"` - 3-digit hex with hash (expands to RRGGBB)
    /// - `"RGB"` - 3-digit hex without hash (expands to RRGGBB)
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_rusty::Color;
    ///
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// let green = Color::from_hex("00FF00").unwrap();
    /// let blue = Color::from_hex("#00F").unwrap();
    /// let white = Color::from_hex("FFF").unwrap();
    ///
    /// assert_eq!(red, Color::rgb(255, 0, 0));
    /// assert_eq!(green, Color::rgb(0, 255, 0));
    /// assert_eq!(blue, Color::rgb(0, 0, 255));
    /// assert_eq!(white, Color::rgb(255, 255, 255));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `HexParseError` if the string is not a valid hex color format.
    pub fn from_hex(hex: &str) -> Result<Self, HexParseError> {
        // Remove '#' prefix if present
        let hex = hex.strip_prefix('#').unwrap_or(hex);

        match hex.len() {
            3 => {
                // 3-digit format: RGB -> RRGGBB
                let r = parse_hex_digit(hex.as_bytes()[0])?;
                let g = parse_hex_digit(hex.as_bytes()[1])?;
                let b = parse_hex_digit(hex.as_bytes()[2])?;

                // Expand: F -> FF (15 -> 255)
                Ok(Color::Rgb(r * 17, g * 17, b * 17))
            }
            6 => {
                // 6-digit format: RRGGBB
                let r = parse_hex_byte(&hex[0..2])?;
                let g = parse_hex_byte(&hex[2..4])?;
                let b = parse_hex_byte(&hex[4..6])?;

                Ok(Color::Rgb(r, g, b))
            }
            _ => Err(HexParseError::InvalidLength(hex.len())),
        }
    }
}

/// Error type for hex color parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HexParseError {
    /// Invalid hex string length (expected 3 or 6 characters)
    InvalidLength(usize),
    /// Invalid hex character
    InvalidHexChar(char),
}

impl fmt::Display for HexParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HexParseError::InvalidLength(len) => {
                write!(f, "Invalid hex color length: {} (expected 3 or 6)", len)
            }
            HexParseError::InvalidHexChar(ch) => {
                write!(f, "Invalid hex character: '{}'", ch)
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HexParseError {}

impl FromStr for Color {
    type Err = HexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::from_hex(s)
    }
}

// Helper function to parse a single hex digit (0-F)
fn parse_hex_digit(byte: u8) -> Result<u8, HexParseError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(HexParseError::InvalidHexChar(byte as char)),
    }
}

// Helper function to parse a hex byte (00-FF)
fn parse_hex_byte(hex: &str) -> Result<u8, HexParseError> {
    if hex.len() != 2 {
        return Err(HexParseError::InvalidLength(hex.len()));
    }

    let high = parse_hex_digit(hex.as_bytes()[0])?;
    let low = parse_hex_digit(hex.as_bytes()[1])?;

    Ok(high * 16 + low)
}

/// Trait for types that can be drawn on.
///
/// This trait is implemented by both `Matrix1` (grayscale) and `Matrix3` (RGB)
/// to provide a unified drawing API.
pub trait DrawTarget {
    /// Returns the width of the drawing target.
    fn width(&self) -> usize;

    /// Returns the height of the drawing target.
    fn height(&self) -> usize;

    /// Sets a pixel at the specified location with the given color.
    ///
    /// Returns true if the pixel was set successfully, false if coordinates are out of bounds.
    fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) -> bool;
}

impl DrawTarget for Matrix1 {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) -> bool {
        self.set_pixel(x, y, color.to_gray())
    }
}

impl DrawTarget for Matrix3 {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) -> bool {
        let (r, g, b) = color.to_rgb();
        self.set_pixel(x, y, r, g, b)
    }
}

/// Draws a rectangle on any image type (Matrix1 or Matrix3).
///
/// # Arguments
///
/// * `image` - The image to draw on (Matrix1 or Matrix3)
/// * `x` - X coordinate of the rectangle's center
/// * `y` - Y coordinate of the rectangle's center
/// * `width` - Width of the rectangle
/// * `height` - Height of the rectangle
/// * `rotation` - Rotation angle in degrees (clockwise)
/// * `stroke_width` - Width of the outline (0 for no outline)
/// * `stroke_color` - Color of the outline (None for no outline)
/// * `fill_color` - Color to fill the rectangle (None for no fill)
///
/// # Examples
///
/// ```
/// use cv_rusty::{Matrix3, Matrix1, draw_rectangle, Color};
///
/// // Draw on RGB image
/// let mut rgb_image = Matrix3::zeros(640, 480);
/// draw_rectangle(
///     &mut rgb_image,
///     320.0, 240.0,
///     100.0, 60.0,
///     45.0,
///     2,
///     Some(Color::rgb(0, 0, 0)),
///     Some(Color::rgb(255, 0, 0))
/// );
///
/// // Draw on grayscale image
/// let mut gray_image = Matrix1::zeros(640, 480);
/// draw_rectangle(
///     &mut gray_image,
///     320.0, 240.0,
///     100.0, 60.0,
///     0.0,
///     2,
///     Some(Color::gray(255)),
///     Some(Color::gray(100))
/// );
/// ```
pub fn draw_rectangle<T: DrawTarget>(
    image: &mut T,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    stroke_width: u32,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
) {
    // Draw fill first (if any)
    if let Some(color) = fill_color {
        draw_filled_rectangle(image, x, y, width, height, rotation, color);
    }

    // Draw stroke on top (if any)
    if stroke_width > 0 && stroke_color.is_some() {
        draw_rectangle_outline(
            image,
            x,
            y,
            width,
            height,
            rotation,
            stroke_width,
            stroke_color.unwrap(),
        );
    }
}

/// Draws a circle on any image type (Matrix1 or Matrix3).
///
/// # Arguments
///
/// * `image` - The image to draw on (Matrix1 or Matrix3)
/// * `x` - X coordinate of the circle's center
/// * `y` - Y coordinate of the circle's center
/// * `radius` - Radius of the circle
/// * `stroke_width` - Width of the outline (0 for no outline)
/// * `stroke_color` - Color of the outline (None for no outline)
/// * `fill_color` - Color to fill the circle (None for no fill)
///
/// # Examples
///
/// ```
/// use cv_rusty::{Matrix3, Matrix1, draw_circle, Color};
///
/// // Draw on RGB image
/// let mut rgb_image = Matrix3::zeros(640, 480);
/// draw_circle(
///     &mut rgb_image,
///     320.0, 240.0,
///     50.0,
///     3,
///     Some(Color::rgb(255, 255, 255)),
///     Some(Color::rgb(0, 0, 255))
/// );
///
/// // Draw on grayscale image
/// let mut gray_image = Matrix1::zeros(640, 480);
/// draw_circle(
///     &mut gray_image,
///     320.0, 240.0,
///     50.0,
///     3,
///     Some(Color::gray(255)),
///     Some(Color::gray(100))
/// );
/// ```
pub fn draw_circle<T: DrawTarget>(
    image: &mut T,
    x: f32,
    y: f32,
    radius: f32,
    stroke_width: u32,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
) {
    // Draw fill first (if any)
    if let Some(color) = fill_color {
        draw_filled_circle(image, x, y, radius, color);
    }

    // Draw stroke on top (if any)
    if stroke_width > 0 && stroke_color.is_some() {
        draw_circle_outline(image, x, y, radius, stroke_width, stroke_color.unwrap());
    }
}

// Helper function to check if a point is inside a rotated rectangle
fn point_in_rotated_rect(
    px: f32,
    py: f32,
    cx: f32,
    cy: f32,
    width: f32,
    height: f32,
    rotation: f32,
) -> bool {
    // Convert rotation to radians
    let angle = rotation.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    // Translate point to rectangle's coordinate system
    let dx = px - cx;
    let dy = py - cy;

    // Rotate point back to axis-aligned position
    let local_x = dx * cos_a + dy * sin_a;
    let local_y = -dx * sin_a + dy * cos_a;

    // Check if point is inside axis-aligned rectangle
    local_x.abs() <= width / 2.0 && local_y.abs() <= height / 2.0
}

// Helper function to draw a filled rectangle
fn draw_filled_rectangle<T: DrawTarget>(
    image: &mut T,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    color: Color,
) {
    // Calculate bounding box
    let half_diag = ((width * width + height * height) / 4.0).sqrt();
    let min_x = (x - half_diag).max(0.0) as usize;
    let max_x = (x + half_diag).min(image.width() as f32) as usize;
    let min_y = (y - half_diag).max(0.0) as usize;
    let max_y = (y + half_diag).min(image.height() as f32) as usize;

    // Scan and fill pixels inside the rotated rectangle
    for py in min_y..max_y {
        for px in min_x..max_x {
            if point_in_rotated_rect(
                px as f32 + 0.5,
                py as f32 + 0.5,
                x,
                y,
                width,
                height,
                rotation,
            ) {
                image.set_pixel_color(px, py, color);
            }
        }
    }
}

// Helper function to draw rectangle outline
fn draw_rectangle_outline<T: DrawTarget>(
    image: &mut T,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    rotation: f32,
    stroke_width: u32,
    color: Color,
) {
    let angle = rotation.to_radians();
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    // Calculate the four corners
    let hw = width / 2.0;
    let hh = height / 2.0;

    let corners = [(-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)];

    let rotated_corners: Vec<(f32, f32)> = corners
        .iter()
        .map(|(lx, ly)| {
            let rx = x + lx * cos_a - ly * sin_a;
            let ry = y + lx * sin_a + ly * cos_a;
            (rx, ry)
        })
        .collect();

    // Draw four lines connecting the corners
    for i in 0..4 {
        let (x1, y1) = rotated_corners[i];
        let (x2, y2) = rotated_corners[(i + 1) % 4];
        draw_thick_line(image, x1, y1, x2, y2, stroke_width, color);
    }
}

// Helper function to draw a filled circle
fn draw_filled_circle<T: DrawTarget>(image: &mut T, cx: f32, cy: f32, radius: f32, color: Color) {
    let r_squared = radius * radius;

    let min_x = (cx - radius).max(0.0) as usize;
    let max_x = (cx + radius).min(image.width() as f32) as usize;
    let min_y = (cy - radius).max(0.0) as usize;
    let max_y = (cy + radius).min(image.height() as f32) as usize;

    for py in min_y..max_y {
        for px in min_x..max_x {
            let dx = px as f32 + 0.5 - cx;
            let dy = py as f32 + 0.5 - cy;
            if dx * dx + dy * dy <= r_squared {
                image.set_pixel_color(px, py, color);
            }
        }
    }
}

// Helper function to draw circle outline
fn draw_circle_outline<T: DrawTarget>(
    image: &mut T,
    cx: f32,
    cy: f32,
    radius: f32,
    stroke_width: u32,
    color: Color,
) {
    let inner_r_squared = (radius - stroke_width as f32 / 2.0).max(0.0).powi(2);
    let outer_r_squared = (radius + stroke_width as f32 / 2.0).powi(2);

    let margin = radius + stroke_width as f32;
    let min_x = (cx - margin).max(0.0) as usize;
    let max_x = (cx + margin).min(image.width() as f32) as usize;
    let min_y = (cy - margin).max(0.0) as usize;
    let max_y = (cy + margin).min(image.height() as f32) as usize;

    for py in min_y..max_y {
        for px in min_x..max_x {
            let dx = px as f32 + 0.5 - cx;
            let dy = py as f32 + 0.5 - cy;
            let dist_squared = dx * dx + dy * dy;

            if dist_squared >= inner_r_squared && dist_squared <= outer_r_squared {
                image.set_pixel_color(px, py, color);
            }
        }
    }
}

// Helper function to draw a thick line using Bresenham's algorithm
fn draw_thick_line<T: DrawTarget>(
    image: &mut T,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    thickness: u32,
    color: Color,
) {
    // Use Bresenham's line algorithm
    let mut x1 = x1;
    let mut y1 = y1;
    let x2 = x2;
    let y2 = y2;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1.0 } else { -1.0 };
    let sy = if y1 < y2 { 1.0 } else { -1.0 };
    let mut err = dx - dy;

    loop {
        // Draw a circle at this point for thickness
        let half_thick = thickness as f32 / 2.0;
        for dy in -(half_thick as i32)..=(half_thick as i32) {
            for dx in -(half_thick as i32)..=(half_thick as i32) {
                if (dx * dx + dy * dy) as f32 <= half_thick * half_thick {
                    let px = (x1 as i32 + dx) as usize;
                    let py = (y1 as i32 + dy) as usize;
                    if px < image.width() && py < image.height() {
                        image.set_pixel_color(px, py, color);
                    }
                }
            }
        }

        if (x1 - x2).abs() < 0.5 && (y1 - y2).abs() < 0.5 {
            break;
        }

        let e2 = 2.0 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversions() {
        let gray = Color::gray(128);
        assert_eq!(gray.to_gray(), 128);
        assert_eq!(gray.to_rgb(), (128, 128, 128));

        let rgb = Color::rgb(255, 128, 64);
        assert_eq!(rgb.to_rgb(), (255, 128, 64));
    }

    #[test]
    fn test_hex_parsing_6_digit() {
        // With hash
        assert_eq!(Color::from_hex("#FF0000").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("#00FF00").unwrap(), Color::rgb(0, 255, 0));
        assert_eq!(Color::from_hex("#0000FF").unwrap(), Color::rgb(0, 0, 255));
        assert_eq!(
            Color::from_hex("#FFFFFF").unwrap(),
            Color::rgb(255, 255, 255)
        );
        assert_eq!(Color::from_hex("#000000").unwrap(), Color::rgb(0, 0, 0));

        // Without hash
        assert_eq!(Color::from_hex("FF0000").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("00FF00").unwrap(), Color::rgb(0, 255, 0));
        assert_eq!(Color::from_hex("0000FF").unwrap(), Color::rgb(0, 0, 255));

        // Lowercase
        assert_eq!(Color::from_hex("#ff0000").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("00ff00").unwrap(), Color::rgb(0, 255, 0));

        // Mixed case
        assert_eq!(Color::from_hex("#Ff00fF").unwrap(), Color::rgb(255, 0, 255));
    }

    #[test]
    fn test_hex_parsing_3_digit() {
        // With hash
        assert_eq!(Color::from_hex("#F00").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("#0F0").unwrap(), Color::rgb(0, 255, 0));
        assert_eq!(Color::from_hex("#00F").unwrap(), Color::rgb(0, 0, 255));
        assert_eq!(Color::from_hex("#FFF").unwrap(), Color::rgb(255, 255, 255));
        assert_eq!(Color::from_hex("#000").unwrap(), Color::rgb(0, 0, 0));

        // Without hash
        assert_eq!(Color::from_hex("F00").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("0F0").unwrap(), Color::rgb(0, 255, 0));
        assert_eq!(Color::from_hex("00F").unwrap(), Color::rgb(0, 0, 255));

        // Lowercase
        assert_eq!(Color::from_hex("#f00").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("0f0").unwrap(), Color::rgb(0, 255, 0));

        // Test expansion: 3 -> 33, 9 -> 99, etc.
        assert_eq!(Color::from_hex("#369").unwrap(), Color::rgb(51, 102, 153));
        assert_eq!(Color::from_hex("#ABC").unwrap(), Color::rgb(170, 187, 204));
    }

    #[test]
    fn test_hex_parsing_errors() {
        // Invalid length
        assert!(Color::from_hex("").is_err());
        assert!(Color::from_hex("#").is_err());
        assert!(Color::from_hex("FF").is_err());
        assert!(Color::from_hex("#FF").is_err());
        assert!(Color::from_hex("FFFF").is_err());
        assert!(Color::from_hex("#FFFFFFF").is_err());

        // Invalid characters
        assert!(Color::from_hex("GGGGGG").is_err());
        assert!(Color::from_hex("#ZZZZZ").is_err());
        assert!(Color::from_hex("FF00GG").is_err());
        assert!(Color::from_hex("#12345Z").is_err());
    }

    #[test]
    fn test_hex_from_str() {
        // Test FromStr trait implementation
        let red: Color = "#FF0000".parse().unwrap();
        assert_eq!(red, Color::rgb(255, 0, 0));

        let green: Color = "00FF00".parse().unwrap();
        assert_eq!(green, Color::rgb(0, 255, 0));

        let blue: Color = "#00F".parse().unwrap();
        assert_eq!(blue, Color::rgb(0, 0, 255));

        // Test error case
        let result: Result<Color, _> = "invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_common_colors() {
        // Test common named colors
        assert_eq!(Color::from_hex("#FF0000").unwrap(), Color::rgb(255, 0, 0)); // Red
        assert_eq!(Color::from_hex("#00FF00").unwrap(), Color::rgb(0, 255, 0)); // Green
        assert_eq!(Color::from_hex("#0000FF").unwrap(), Color::rgb(0, 0, 255)); // Blue
        assert_eq!(Color::from_hex("#FFFF00").unwrap(), Color::rgb(255, 255, 0)); // Yellow
        assert_eq!(Color::from_hex("#FF00FF").unwrap(), Color::rgb(255, 0, 255)); // Magenta
        assert_eq!(Color::from_hex("#00FFFF").unwrap(), Color::rgb(0, 255, 255)); // Cyan
        assert_eq!(Color::from_hex("#000000").unwrap(), Color::rgb(0, 0, 0)); // Black
        assert_eq!(
            Color::from_hex("#FFFFFF").unwrap(),
            Color::rgb(255, 255, 255)
        ); // White
        assert_eq!(
            Color::from_hex("#808080").unwrap(),
            Color::rgb(128, 128, 128)
        ); // Gray
    }

    #[test]
    fn test_draw_rectangle_matrix3() {
        let mut image = Matrix3::zeros(100, 100);
        draw_rectangle(
            &mut image,
            50.0,
            50.0,
            20.0,
            10.0,
            0.0,
            1,
            Some(Color::white()),
            Some(Color::rgb(255, 0, 0)),
        );

        // Check that some pixels were modified
        let center = image.get_pixel(50, 50).unwrap();
        assert_eq!(center, (255, 0, 0));
    }

    #[test]
    fn test_draw_rectangle_matrix1() {
        let mut image = Matrix1::zeros(100, 100);
        draw_rectangle(
            &mut image,
            50.0,
            50.0,
            20.0,
            10.0,
            0.0,
            1,
            Some(Color::white()),
            Some(Color::gray(128)),
        );

        // Check that some pixels were modified
        let center = image.get_pixel(50, 50).unwrap();
        assert_eq!(center, 128);
    }

    #[test]
    fn test_draw_circle_matrix3() {
        let mut image = Matrix3::zeros(100, 100);
        draw_circle(
            &mut image,
            50.0,
            50.0,
            10.0,
            1,
            Some(Color::white()),
            Some(Color::rgb(0, 255, 0)),
        );

        // Check center pixel
        let center = image.get_pixel(50, 50).unwrap();
        assert_eq!(center, (0, 255, 0));
    }

    #[test]
    fn test_draw_circle_matrix1() {
        let mut image = Matrix1::zeros(100, 100);
        draw_circle(
            &mut image,
            50.0,
            50.0,
            10.0,
            1,
            Some(Color::white()),
            Some(Color::gray(200)),
        );

        // Check center pixel
        let center = image.get_pixel(50, 50).unwrap();
        assert_eq!(center, 200);
    }

    #[test]
    fn test_point_in_rotated_rect() {
        // Test axis-aligned rectangle
        assert!(point_in_rotated_rect(
            50.0, 50.0, 50.0, 50.0, 20.0, 10.0, 0.0
        ));
        assert!(!point_in_rotated_rect(
            70.0, 50.0, 50.0, 50.0, 20.0, 10.0, 0.0
        ));

        // Test rotated rectangle (45 degrees)
        assert!(point_in_rotated_rect(
            50.0, 50.0, 50.0, 50.0, 20.0, 10.0, 45.0
        ));
    }

    #[test]
    fn test_draw_target_trait() {
        // Test that both Matrix1 and Matrix3 implement DrawTarget
        let mut rgb = Matrix3::zeros(10, 10);
        let mut gray = Matrix1::zeros(10, 10);

        assert_eq!(rgb.width(), 10);
        assert_eq!(rgb.height(), 10);
        assert_eq!(gray.width(), 10);
        assert_eq!(gray.height(), 10);

        assert!(rgb.set_pixel_color(5, 5, Color::rgb(255, 0, 0)));
        assert!(gray.set_pixel_color(5, 5, Color::gray(128)));
    }
}
