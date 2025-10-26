# Unified `show_image()` API - Changelog

## Version 0.4.0 (TBD)

### Summary

Unified the window display API by consolidating `show_image()` and `show_image_color()` into a single generic function that works with both grayscale (`Matrix1`) and color (`Matrix3`) images through Rust's trait system.

---

## Breaking Changes

### Removed Functions

The following functions have been removed:

- ❌ **`show_image_color()`** - Merged into `show_image()`
- ❌ **`show_and_wait_gray()`** - Merged into `show_and_wait()`

### Migration Required

```rust
// OLD API
show_image_color("Color Window", &color_image)?;
show_image("Gray Window", &gray_image)?;
show_and_wait_gray("Gray", &gray_image)?;

// NEW API
show_image("Color Window", &color_image)?;
show_image("Gray Window", &gray_image)?;
show_and_wait("Gray", &gray_image)?;
```

**Migration Difficulty**: ⭐ Easy (simple find-and-replace)

See [MIGRATION_GUIDE.md](../../MIGRATION_GUIDE.md) for detailed instructions.

---

## New Features

### 1. Unified `show_image()` Function

The `show_image()` function now accepts any type that implements the `Displayable` trait:

```rust
pub fn show_image<T: Displayable>(window_name: &str, image: &T) -> Result<(), WindowError>
```

**Benefits:**

- Single function for all image types
- Type-safe through compile-time checking
- Extensible to custom image types
- Zero-cost abstraction

**Example:**
```rust
use cv_rusty::{Matrix1, Matrix3, show_image};

let color = Matrix3::zeros(640, 480);
let gray = Matrix1::zeros(640, 480);

// Same function works for both!
show_image("Color", &color)?;
show_image("Grayscale", &gray)?;
```

### 2. `Displayable` Trait

New public trait for types that can be displayed in windows:

```rust
pub trait Displayable {
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError>;
}
```

**Implementations:**

- ✅ `Matrix1` (grayscale images)
- ✅ `Matrix3` (color images)
- ✅ Custom types (user-implementable)

**Custom Implementation Example:**
```rust
use cv_rusty::{Displayable, WindowError, show_image};

struct MyImage {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Displayable for MyImage {
    fn to_display_buffer(&self) -> Result<(Vec<u32>, usize, usize), WindowError> {
        if self.width == 0 || self.height == 0 {
            return Err(WindowError::InvalidDimensions);
        }

        let buffer: Vec<u32> = self.data
            .iter()
            .map(|&pixel| {
                let rgb = pixel as u32;
                (rgb << 16) | (rgb << 8) | rgb
            })
            .collect();

        Ok((buffer, self.width, self.height))
    }
}

// Now works with show_image!
let my_img = MyImage { data: vec![128; 640 * 480], width: 640, height: 480 };
show_image("Custom", &my_img)?;
```

### 3. Unified `show_and_wait()` Function

The `show_and_wait()` function is now generic:

```rust
pub fn show_and_wait<T: Displayable>(window_name: &str, image: &T) -> Result<(), WindowError>
```

Works with both `Matrix1` and `Matrix3` without separate functions.

---

## Technical Details

### Implementation Approach

Used Rust's trait system to create a zero-cost abstraction:

1. **Trait Definition**: `Displayable` trait with single method `to_display_buffer()`
2. **Trait Implementations**: For `Matrix1` and `Matrix3`
3. **Generic Function**: `show_image<T: Displayable>()` accepts any displayable type
4. **Compile-Time Resolution**: No runtime overhead, fully optimized

### Performance

- ✅ **Zero-cost abstraction** - No runtime overhead
- ✅ **Identical performance** to previous separate functions
- ✅ **Compile-time dispatch** - No virtual function calls
- ✅ **Inline optimization** - Compiler can fully inline trait methods

### Type Safety

The trait-based approach provides stronger type safety:

```rust
// ✅ Compiles - Matrix3 implements Displayable
show_image("Window", &Matrix3::zeros(640, 480))?;

// ✅ Compiles - Matrix1 implements Displayable
show_image("Window", &Matrix1::zeros(640, 480))?;

// ❌ Compile error - String doesn't implement Displayable
show_image("Window", &"not an image")?;
```

---

## Updated Components

### Source Code

- ✅ `src/window.rs` - Refactored with trait-based implementation
- ✅ `src/lib.rs` - Updated public exports

### Examples

- ✅ `examples/simple_show_image.rs` - Updated to use unified API
- ✅ `examples/window_display_example.rs` - Updated to use unified API
- ✅ `examples/test_window.rs` - Updated to use unified API

### Documentation

- ✅ `README.md` - Updated usage examples
- ✅ `docs/guides/window_display.md` - Comprehensive guide update
- ✅ `docs/quick-reference.md` - All examples updated
- ✅ `docs/changelogs/window_feature_summary.md` - API documentation updated
- ✅ `examples/README.md` - Example descriptions updated
- ✅ `MIGRATION_GUIDE.md` - New comprehensive migration guide

---

## API Comparison

### Before (v0.3.x)

```rust
use cv_rusty::{show_image, show_image_color, show_and_wait, show_and_wait_gray};

// Separate functions for each type
show_image_color("Color", &color_image)?;      // For Matrix3
show_image("Gray", &gray_image)?;               // For Matrix1
show_and_wait("Color", &color_image)?;          // For Matrix3
show_and_wait_gray("Gray", &gray_image)?;       // For Matrix1
```

### After (v0.4.0)

```rust
use cv_rusty::{show_image, show_and_wait};

// Single function for all types
show_image("Color", &color_image)?;    // Works with Matrix3
show_image("Gray", &gray_image)?;      // Works with Matrix1
show_and_wait("Color", &color_image)?; // Works with both
show_and_wait("Gray", &gray_image)?;   // Works with both
```

---

## Advantages

### Developer Experience

1. **Simplified API Surface**

   - Fewer functions to remember
   - No need to choose between variants
   - Single consistent interface

2. **Better Code Readability**
   ```rust
   // Clear and uniform
   show_image("Window 1", &image1)?;
   show_image("Window 2", &image2)?;
   show_image("Window 3", &image3)?;
   ```

3. **Enhanced Type Safety**

   - Compile-time checking
   - Better error messages
   - No runtime type checking

4. **Extensibility**

   - Implement `Displayable` for custom types
   - Works seamlessly with existing API
   - Future-proof design

### Code Maintenance

- Easier to maintain single implementation
- Reduced code duplication
- More flexible for future enhancements
- Follows Rust idioms (traits over overloading)

---

## Testing

All tests and examples pass with the new API:

```bash
# Run checks
cargo check --features window --all-targets

# Build examples
cargo build --example simple_show_image --features window
cargo build --example window_display_example --features window
cargo build --example test_window --features window

# Run examples
cargo run --example simple_show_image --features window
cargo run --example window_display_example --features window
```

---

## Deprecation Timeline

### Version 0.4.0 (This Release)

- ❌ `show_image_color()` removed
- ❌ `show_and_wait_gray()` removed
- ✅ `show_image<T: Displayable>()` replaces both
- ✅ `show_and_wait<T: Displayable>()` unified

### No Deprecation Period

Since this is a clean break with clear migration path:

- Functions removed immediately (not deprecated)
- Migration is mechanical and straightforward
- No behavior changes, only naming unification

---

## Rationale

### Why Unify?

1. **OpenCV Comparison**: Even OpenCV uses a single `imshow()` for all image types
2. **Rust Idioms**: Traits are the Rust way to handle polymorphism
3. **API Clarity**: Single function is simpler than remembering variants
4. **Extensibility**: Users can add their own displayable types
5. **Modern Design**: Leverages Rust's type system effectively

### Design Decisions

- **Trait over Enum**: More flexible, allows user extensions
- **Generic Function**: Zero-cost, compile-time resolution
- **Public Trait**: Enables custom implementations
- **Clean Break**: No deprecated functions to maintain

---

## Future Enhancements

The trait-based design enables future improvements:

1. **Display Options**: Add display configuration through trait methods
2. **Image Formats**: Support additional image formats
3. **Custom Rendering**: Allow custom pixel format conversions
4. **Batch Operations**: Display multiple images efficiently
5. **Animation Support**: Extend trait for animated displays

---

## Credits

This change improves API ergonomics while maintaining full backward compatibility in behavior. The unified design follows Rust best practices and makes the library more intuitive for users coming from other computer vision libraries.

**Key Insight**: In Rust, traits provide superior polymorphism compared to function overloading or separate functions for each type.

---

## Resources

- [Migration Guide](../../MIGRATION_GUIDE.md) - Step-by-step migration instructions
- [Window Display Guide](../guides/window_display.md) - Complete API documentation
- [Quick Reference](../quick-reference.md) - Code examples
- [Examples](../../examples/) - Working code samples

---

## Questions?

If you have questions about this change:

1. Read the [Migration Guide](../../MIGRATION_GUIDE.md)
2. Check the updated [documentation](../guides/window_display.md)
3. Review the [examples](../../examples/)
4. Open an issue for further assistance
