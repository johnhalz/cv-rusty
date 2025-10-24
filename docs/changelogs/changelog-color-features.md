# Color Space Conversion Features - Change Log

## New Features Added

### 1. Single-Channel Matrix (Matrix1)

Added a new `Matrix1` struct for representing grayscale/single-channel images:

- **Struct**: `Matrix1` - Single-channel matrix for grayscale image data
- **Methods**:
  - `new(width, height, data)` - Create from raw grayscale data
  - `zeros(width, height)` - Create zero-initialized matrix
  - `get_pixel(x, y)` - Get pixel value at location
  - `set_pixel(x, y, value)` - Set pixel value at location
  - `width()`, `height()`, `dimensions()` - Get matrix dimensions
  - `data()`, `data_mut()` - Access raw pixel data
  - `into_raw()` - Consume matrix and return raw data

### 2. RGB to Grayscale Conversion

Added multiple methods for converting RGB images to grayscale:

**Methods on Matrix3**:
- `to_grayscale()` - Default luminosity method (recommended)
- `to_grayscale_average()` - Simple average method
- `to_grayscale_lightness()` - Lightness (midpoint) method
- `to_grayscale_with_method(GrayscaleMethod)` - Specify conversion method

**Conversion Algorithms**:
- **Luminosity**: `0.299*R + 0.587*G + 0.114*B` - Accounts for human perception
- **Average**: `(R + G + B) / 3` - Simple arithmetic mean
- **Lightness**: `(max(R,G,B) + min(R,G,B)) / 2` - Midpoint of range

**Enum**: `GrayscaleMethod` with variants:
- `Luminosity`
- `Average`
- `Lightness`

### 3. RGB ↔ HSV Color Space Conversion

Added functions for converting between RGB and HSV color spaces:

- `rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32)`
  - Input: RGB values (0-255)
  - Output: (hue in degrees 0-360°, saturation 0.0-1.0, value 0.0-1.0)

- `hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8)`
  - Input: HSV (hue 0-360°, saturation 0.0-1.0, value 0.0-1.0)
  - Output: RGB values (0-255)

### 4. RGB ↔ HSL Color Space Conversion

Added functions for converting between RGB and HSL color spaces:

- `rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32)`
  - Input: RGB values (0-255)
  - Output: (hue in degrees 0-360°, saturation 0.0-1.0, lightness 0.0-1.0)

- `hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8)`
  - Input: HSL (hue 0-360°, saturation 0.0-1.0, lightness 0.0-1.0)
  - Output: RGB values (0-255)

## New Module

### `color` Module

Created a new `color` module (`src/color.rs`) containing:
- Color space conversion functions
- Grayscale conversion implementations
- Comprehensive test suite for all conversions

All color conversion functionality is **`no_std` compatible** and only requires the `alloc` crate.

## Public API Exports

Updated `lib.rs` to export:
- `Matrix1` - Single-channel matrix struct
- `GrayscaleMethod` - Enum for grayscale conversion methods
- `rgb_to_hsv`, `hsv_to_rgb` - HSV conversion functions
- `rgb_to_hsl`, `hsl_to_rgb` - HSL conversion functions

## Examples

Added new example: `color_conversion_example.rs`

Demonstrates:
- RGB to grayscale conversion with all three methods
- RGB ↔ HSV conversions
- RGB ↔ HSL conversions
- Working with Matrix1 (grayscale images)
- Roundtrip conversion tests

Run with:
```bash
cargo run --example color_conversion_example
```

## Tests

Added comprehensive test coverage:
- Matrix1 creation and pixel access tests
- Grayscale conversion tests for all three methods
- RGB ↔ HSV roundtrip conversion tests
- RGB ↔ HSL roundtrip conversion tests
- Pure color conversion accuracy tests
- Edge case handling

All 20 unit tests pass successfully.

## Documentation

Updated documentation:
- Added inline documentation for all new functions and methods
- Updated README.md with usage examples
- Added doc tests that verify code examples compile and run correctly
- Documented all color space conversion formulas

## Backward Compatibility

All changes are **fully backward compatible**:
- No breaking changes to existing API
- Matrix3 functionality unchanged
- All existing tests continue to pass

## Performance Notes

- Color conversions use `f32` for intermediate calculations
- Grayscale conversions are optimized for performance
- No unnecessary allocations in conversion functions
- All operations are `no_std` compatible

## Use Cases

These features enable:
1. **Image preprocessing** - Convert color images to grayscale for algorithms
2. **Color manipulation** - Adjust hue, saturation, brightness in HSV/HSL space
3. **Computer vision** - Many CV algorithms work on grayscale images
4. **Embedded systems** - Reduce memory usage with single-channel images
5. **Display applications** - Convert between color spaces for different displays
6. **Image analysis** - Separate color components for analysis

## Future Enhancements

Potential additions:
- YUV/YCbCr color space conversions
- LAB color space support
- Batch conversion operations for performance
- SIMD-optimized conversions