# CV Rusty Architecture

## Overview

CV Rusty is a `no_std` compatible computer vision library written in Rust with a focus on performance, safety, and embedded systems support. The library is designed for live computations, real-time image processing applications, and resource-constrained environments.

## Project Structure

```
cv-rusty/
├── src/
│   ├── lib.rs          # Main library entry point
│   ├── matrix.rs       # Matrix data structures
│   └── io.rs           # Image I/O operations
├── examples/
│   └── read_jpeg_example.rs
├── Cargo.toml
└── README.md
```

## Module Design

### `lib.rs`

The main library entry point configured for `no_std` compatibility. It uses feature flags to conditionally compile std-dependent code.

**Key Attributes:**
- `#![cfg_attr(not(feature = "std"), no_std)]` - Enables `no_std` when std feature is disabled
- `extern crate alloc` - Required for heap allocations in `no_std` environments

**Public API:**
- `Matrix3` - Three-channel image matrix (always available)
- `read_jpeg` - JPEG file reading function (only with `std` feature)

### `matrix.rs`

Contains the core data structure for representing multi-channel image data. This module is fully `no_std` compatible and only requires the `alloc` crate.

**`no_std` Compatibility:**
- Uses `core::fmt` instead of `std::fmt`
- Imports `Vec` from `alloc::vec::Vec` when `std` is not available
- Imports `vec!` macro from `alloc::vec` for `no_std` environments

#### `Matrix3`

A three-channel matrix specifically designed for RGB image representation.

**Design Decisions:**
- **Contiguous Memory Layout**: Data is stored in a single `Vec<u8>` for cache efficiency
- **Interleaved Channels**: RGB values are stored as `[R, G, B, R, G, B, ...]` for better spatial locality
- **Row-Major Order**: Standard image representation order (y * width + x)
- **Bounds Checking**: All pixel access methods return `Option` or `bool` to prevent panics

**Memory Layout:**
```
Pixel (0,0): [R, G, B] | Pixel (1,0): [R, G, B] | ... | Pixel (W-1,0): [R, G, B]
Pixel (0,1): [R, G, B] | Pixel (1,1): [R, G, B] | ... | Pixel (W-1,1): [R, G, B]
...
Pixel (0,H-1): [R, G, B] | ... | Pixel (W-1,H-1): [R, G, B]
```

**Index Calculation:**
```rust
let pixel_index = (y * width + x) * 3;
let r = data[pixel_index];
let g = data[pixel_index + 1];
let b = data[pixel_index + 2];
```

### `io.rs`

Handles image file I/O operations, currently supporting JPEG format. This module is only compiled when the `std` feature is enabled, as it requires file system access.

**Compilation Guard:**
- Only included when `#[cfg(feature = "std")]` is true
- Requires `std::fs`, `std::io`, and `std::path`

#### `ImageError`

Custom error type for image operations with variants:
- `Io(io::Error)` - File system errors
- `JpegDecode(String)` - JPEG decoding errors
- `UnsupportedFormat(String)` - Unsupported pixel format errors

#### `read_jpeg(path)`

Reads JPEG files and converts them to RGB `Matrix3`.

**Supported Input Formats:**
1. **RGB24** - Direct passthrough (most common)
2. **L8 (Grayscale)** - Converted by duplicating channel: `[G] -> [G, G, G]`
3. **CMYK32** - Converted using formula:
   ```
   R = (1 - C) * (1 - K) * 255
   G = (1 - M) * (1 - K) * 255
   B = (1 - Y) * (1 - K) * 255
   ```

**Processing Pipeline:**
1. Open file with buffered reader
2. Create JPEG decoder
3. Decode image to raw pixels
4. Extract metadata (width, height, pixel format)
5. Convert to RGB if necessary
6. Construct `Matrix3` with RGB data

## Dependencies

### `jpeg-decoder` (v0.3)
- **Purpose**: JPEG image decoding
- **Features**: Supports multiple pixel formats, hardware acceleration
- **License**: MIT/Apache-2.0
- **Availability**: Optional, only included with `std` feature
- **Note**: This dependency requires `std`, so JPEG reading is not available in `no_std` environments

## Feature Flags

### `std` (default)
Enables standard library support, including:
- File I/O operations (`io` module)
- JPEG reading functionality
- `std::error::Error` trait implementations

### `alloc`
Enables heap allocation support (required for core functionality):
- `Vec<u8>` for storing pixel data
- Dynamic memory allocation

**Feature Dependencies:**
```toml
[features]
default = ["std"]
std = ["jpeg-decoder"]
alloc = []
```

## Design Principles

1. **`no_std` First**: Core functionality works without standard library
2. **Zero-Copy When Possible**: Minimize data copying during operations
3. **Memory Safety**: Use Rust's type system to prevent buffer overflows
4. **Ergonomic API**: Simple, intuitive interfaces for common operations
5. **Performance**: Design for real-time processing applications
6. **Extensibility**: Modular design allows easy addition of new formats and operations
7. **Embedded Ready**: Suitable for microcontrollers and resource-constrained systems

## Performance Considerations

### Memory Layout
- Contiguous allocation reduces cache misses
- Interleaved channels improve spatial locality for pixel operations
- Single allocation per image minimizes heap fragmentation

### Bounds Checking
- All pixel access is bounds-checked at compile time or returns `Option`
- No unsafe code in current implementation
- Trade-off: slight overhead for safety guarantees

### Future Optimizations
- SIMD operations for bulk pixel processing
- Parallel processing with rayon for large images (with `std` feature)
- Memory pooling for repeated allocations
- Optional unsafe fast paths for performance-critical code
- Hardware accelerator support for embedded systems
- Zero-allocation operations where possible

## `no_std` Support

### Requirements
The core library (`Matrix3` and related operations) only requires:
- `core` - Rust's core library (always available)
- `alloc` - For heap allocations (`Vec`)

### Limitations in `no_std`
Without the `std` feature, the following are not available:
- File I/O operations (`read_jpeg`, future `write_jpeg`)
- `std::error::Error` trait (we use custom error types)
- Threading/parallelization

### Embedded Use Cases
The `no_std` design enables use in:
- **Microcontrollers** (ARM Cortex-M, RISC-V)
- **Real-time Operating Systems** (RTOS)
- **Bare-metal environments**
- **FPGA/ASIC soft processors**
- **Bootloaders and firmware**

### Memory Considerations
In `no_std` environments:
- Heap allocation requires a global allocator to be configured
- Large images may exceed available RAM on constrained devices
- Consider using smaller images or streaming processing
- Future: Support for static allocation and external memory

## Testing Strategy

### Unit Tests
- Located inline with source code using `#[cfg(test)]`
- Test coverage includes:
  - Matrix creation and initialization
  - Pixel access (get/set)
  - Boundary conditions
  - Error handling

### Doc Tests
- Embedded in documentation comments
- Ensures examples in docs remain valid
- Run automatically with `cargo test`

### Integration Tests
- Examples serve as integration tests
- Real-world usage patterns

## Future Architecture Plans

### Planned Modules

1. **`color.rs`** - Color space conversions (RGB ↔ HSV, LAB, YUV) - `no_std` compatible
2. **`transform.rs`** - Geometric transformations (resize, rotate, crop) - `no_std` compatible
3. **`filter.rs`** - Image filtering (blur, sharpen, edge detection) - `no_std` compatible
4. **`features.rs`** - Feature detection and extraction - `no_std` compatible
5. **`codec/`** - Additional image format support (PNG, BMP, TIFF) - requires `std` feature
6. **`hw/`** - Hardware accelerator interfaces for embedded systems - `no_std` compatible

### Trait System

Future implementation of traits for polymorphism:
```rust
pub trait ImageMatrix {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn channels(&self) -> usize;
}

pub trait ImageReader {
    fn read<P: AsRef<Path>>(path: P) -> Result<Matrix3, ImageError>;
}

pub trait ImageWriter {
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError>;
}
```

### Generic Matrix Types

Potential expansion to support different channel counts and data types:
- `Matrix<T, C>` - Generic over data type and channel count
- `Matrix1<T>` - Single channel (grayscale)
- `Matrix3<T>` - Three channels (RGB)
- `Matrix4<T>` - Four channels (RGBA)

## Contributing Guidelines

When adding new features:
1. Follow existing module organization patterns
2. **Design for `no_std` first** - use `core` and `alloc` when possible
3. Add comprehensive tests (test with and without `std` feature)
4. Document all public APIs with examples
5. Consider performance implications, especially for embedded systems
6. Maintain memory safety without unsafe code (unless absolutely necessary)
7. Use feature flags appropriately (`std` for I/O, keep core logic `no_std`)
8. Update this architecture document

### Testing Strategy for `no_std`

When testing `no_std` compatibility:
```bash
# Build without std
cargo build --no-default-features

# Build with std
cargo build --features std

# Run tests (requires std for test framework)
cargo test
```

## Version History

- **v0.1.0** - Initial release with `no_std` support, JPEG reading (with `std` feature), and Matrix3 support