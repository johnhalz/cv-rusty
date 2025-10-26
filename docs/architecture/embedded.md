# Embedded Systems Guide for CV Rusty

This guide explains how to use CV Rusty in embedded and `no_std` environments.

## Overview

CV Rusty is designed to work in resource-constrained environments without the Rust standard library. The core functionality only requires the `alloc` crate for heap allocations.

## Requirements

### Minimum Requirements

- **Rust Edition**: 2021 or later
- **Memory**: Heap allocator (`alloc` crate)
- **No standard library**: Works with `#![no_std]`

### Memory Usage

For a `Matrix3` image, the memory requirements are:
```
Memory = width × height × 3 bytes + struct overhead (~24 bytes)
```

**Examples:**

- 320×240 image: ~230 KB
- 640×480 image: ~921 KB
- 160×120 image: ~57 KB
- 80×60 image: ~14 KB

## Setup

### 1. Configure Cargo.toml

```toml
[dependencies]
cv-rusty = { version = "0.1.0", default-features = false }

# You'll need a global allocator for embedded systems
# Choose one appropriate for your platform:
embedded-alloc = "0.5"  # For simple embedded allocators
```

### 2. Configure Your Main File

```rust
#![no_std]
#![no_main]

// Import alloc for heap allocations
extern crate alloc;

// Set up a global allocator (example using embedded-alloc)
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// Initialize the heap in your main/init function
pub fn init() {
    const HEAP_SIZE: usize = 64 * 1024; // 64 KB heap
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
```

## Usage Patterns

### 1. Creating Images from Sensor Data

```rust
use cv_rusty::Matrix3;

fn capture_from_camera(width: usize, height: usize) -> Matrix3 {
    let mut image = Matrix3::zeros(width, height);

    // Read from camera sensor (pseudo-code)
    for y in 0..height {
        for x in 0..width {
            let (r, g, b) = read_pixel_from_sensor(x, y);
            image.set_pixel(x, y, r, g, b);
        }
    }

    image
}
```

### 2. Processing Images In-Place

```rust
use cv_rusty::Matrix3;

fn adjust_brightness(image: &mut Matrix3, factor: f32) {
    let data = image.data_mut();

    for pixel in data.iter_mut() {
        let new_val = (*pixel as f32 * factor).min(255.0) as u8;
        *pixel = new_val;
    }
}
```

### 3. Sending to Display via SPI

```rust
use cv_rusty::Matrix3;

fn send_to_display(image: &Matrix3, spi: &mut SpiInterface) {
    // Get raw RGB data
    let data = image.data();

    // Send to display in chunks to avoid large stack usage
    const CHUNK_SIZE: usize = 256;
    for chunk in data.chunks(CHUNK_SIZE) {
        spi.write(chunk).ok();
    }
}
```

### 4. Low-Memory Streaming Processing

For very constrained systems, process images in chunks:

```rust
use cv_rusty::Matrix3;

fn process_image_rows(width: usize, total_height: usize) {
    // Process 10 rows at a time
    const ROWS_PER_BATCH: usize = 10;

    for start_row in (0..total_height).step_by(ROWS_PER_BATCH) {
        let rows = ROWS_PER_BATCH.min(total_height - start_row);
        let mut batch = Matrix3::zeros(width, rows);

        // Fill batch from sensor
        for y in 0..rows {
            for x in 0..width {
                let (r, g, b) = read_pixel_from_sensor(x, start_row + y);
                batch.set_pixel(x, y, r, g, b);
            }
        }

        // Process this batch
        process_batch(&mut batch);

        // Send to output
        output_batch(&batch);
    }
}

fn process_batch(batch: &mut Matrix3) {
    // Apply your image processing here
}

fn output_batch(batch: &Matrix3) {
    // Send to display or storage
}
```

## Platform-Specific Examples

### ARM Cortex-M (STM32, nRF52, etc.)

```rust
#![no_std]
#![no_main]

extern crate alloc;
use panic_halt as _;
use cortex_m_rt::entry;
use embedded_alloc::Heap;
use cv_rusty::Matrix3;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize heap
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    // Initialize peripherals (camera, display, etc.)
    let mut camera = init_camera();
    let mut display = init_display();

    loop {
        // Capture small image (80x60 = ~14KB)
        let image = capture_image(&mut camera, 80, 60);

        // Process image
        let processed = apply_filters(&image);

        // Display result
        send_to_display(&processed, &mut display);
    }
}

fn capture_image(camera: &mut Camera, width: usize, height: usize) -> Matrix3 {
    let mut image = Matrix3::zeros(width, height);
    // Capture logic...
    image
}

fn apply_filters(image: &Matrix3) -> Matrix3 {
    // Clone and process
    let mut result = image.clone();
    // Apply your filters...
    result
}
```

### ESP32 (Embedded Rust)

```rust
#![no_std]
#![no_main]

extern crate alloc;
use esp_backtrace as _;
use esp_println::println;
use esp_alloc as _;
use cv_rusty::Matrix3;

#[entry]
fn main() -> ! {
    // ESP32 provides its own allocator
    esp_alloc::heap_allocator!(72 * 1024); // 72 KB heap

    println!("CV Rusty on ESP32");

    // ESP32-CAM can handle larger images (up to 640x480 on ESP32-S3)
    let image = Matrix3::zeros(320, 240);
    println!("Created {}x{} image", image.width(), image.height());

    loop {
        // Process images from ESP32-CAM
    }
}
```

### RISC-V

```rust
#![no_std]
#![no_main]

extern crate alloc;
use riscv_rt::entry;
use embedded_alloc::Heap;
use cv_rusty::Matrix3;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initialize heap
    const HEAP_SIZE: usize = 64 * 1024;
    static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }

    // Your RISC-V application
    let image = Matrix3::zeros(160, 120);

    loop {
        // Process images
    }
}
```

## Common Pitfalls and Solutions

### 1. Stack Overflow

**Problem**: Creating large images on the stack
```rust
// DON'T: This might overflow the stack
let image = Matrix3::zeros(640, 480); // ~921 KB allocated
```

**Solution**: This is already safe! `Matrix3` uses `Vec` internally, which allocates on the heap.

### 2. Heap Exhaustion

**Problem**: Not enough heap space
```rust
// Might fail if heap is too small
let image = Matrix3::zeros(640, 480); // Needs ~921 KB
```

**Solution**: Either increase heap size or use smaller images
```rust
// Use smaller images for constrained devices
let image = Matrix3::zeros(160, 120); // Only ~57 KB

// Or process in chunks (see streaming example above)
```

### 3. Missing Allocator

**Problem**: Linker error about missing `__rust_alloc`

**Solution**: Always configure a global allocator:
```rust
use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();
```

### 4. Debug Formatting

**Problem**: Using `println!` or `format!` in `no_std`

**Solution**: Use platform-specific printing (e.g., `defmt`, `esp_println`, or semihosting)
```rust
// Instead of println!, use platform-specific macros
use defmt::info;
info!("Image size: {}x{}", image.width(), image.height());
```

## Memory Optimization Tips

### 1. Reuse Image Buffers

```rust
// Keep one image buffer and reuse it
static mut IMAGE_BUFFER: Option<Matrix3> = None;

fn get_image_buffer(width: usize, height: usize) -> &'static mut Matrix3 {
    unsafe {
        if IMAGE_BUFFER.is_none() {
            IMAGE_BUFFER = Some(Matrix3::zeros(width, height));
        }
        IMAGE_BUFFER.as_mut().unwrap()
    }
}
```

### 2. In-Place Operations

```rust
// Modify the image data directly instead of creating new images
fn process_inplace(image: &mut Matrix3) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            if let Some((r, g, b)) = image.get_pixel(x, y) {
                // Process and write back
                let new_r = (r as u16 * 2).min(255) as u8;
                image.set_pixel(x, y, new_r, g, b);
            }
        }
    }
}
```

### 3. Use Smaller Color Depths

For very constrained systems, consider:
- Downsampling images before processing
- Using grayscale by keeping only one channel
- Processing regions of interest (ROI) instead of full images

```rust
// Extract a region of interest
fn extract_roi(src: &Matrix3, x: usize, y: usize, w: usize, h: usize) -> Matrix3 {
    let mut roi = Matrix3::zeros(w, h);
    for dy in 0..h {
        for dx in 0..w {
            if let Some((r, g, b)) = src.get_pixel(x + dx, y + dy) {
                roi.set_pixel(dx, dy, r, g, b);
            }
        }
    }
    roi
}
```

## Performance Considerations

### Typical Performance (ARM Cortex-M4 @ 80 MHz)

- Creating 160×120 image: ~1 ms
- Pixel access: ~10 ns per pixel
- Full image iteration: ~5-10 ms for 160×120

### Optimization Strategies

1. **Batch Operations**: Process multiple pixels at once
2. **Cache Locality**: Access pixels in row-major order
3. **Minimize Allocations**: Reuse buffers when possible
4. **Direct Data Access**: Use `data()` and `data_mut()` for bulk operations

## Testing on Host System

You can test your embedded code on your development machine:

```bash
# Build for embedded target
cargo build --target thumbv7em-none-eabihf --no-default-features

# Test on host (with std for test framework)
cargo test
```

## Integration with Hardware Accelerators

Many embedded systems have hardware acceleration for image processing:

```rust
// Example: Offload operations to DMA or GPU
fn hardware_accelerated_copy(image: &Matrix3, hw_buffer: &mut [u8]) {
    let data = image.data();

    // Use DMA to copy data
    dma_transfer(data, hw_buffer);

    // Trigger hardware processor
    trigger_hw_processor();
}
```

## Additional Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [rust-embedded/awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust)
- [Allocators for embedded systems](https://docs.rust-embedded.org/book/collections/index.html)

## Support

For questions about using CV Rusty in embedded systems, please:

1. Check the examples in `examples/no_std_example.rs`
2. Review this guide
3. Open an issue on GitHub with your platform details

## License

Same as the main CV Rusty library (MIT).
