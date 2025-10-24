use cv_rusty::io::read_jpeg;

fn main() {
    // Example usage of reading a JPEG image
    println!("CV Rusty - JPEG Reader Example\n");

    // Try to read a JPEG file from the command line argument
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_jpeg_file>", args[0]);
        eprintln!("\nExample: {} photo.jpg", args[0]);
        std::process::exit(1);
    }

    let jpeg_path = &args[1];
    println!("Reading JPEG file: {}", jpeg_path);

    match read_jpeg(jpeg_path) {
        Ok(image) => {
            println!("✓ Successfully loaded image!");
            println!("  Dimensions: {}x{}", image.width(), image.height());
            println!("  Total pixels: {}", image.width() * image.height());
            println!("  Data size: {} bytes", image.data().len());

            // Display some pixel information
            if let Some((r, g, b)) = image.get_pixel(0, 0) {
                println!("\n  Top-left pixel (0,0): RGB({}, {}, {})", r, g, b);
            }

            if image.width() > 0 && image.height() > 0 {
                let mid_x = image.width() / 2;
                let mid_y = image.height() / 2;
                if let Some((r, g, b)) = image.get_pixel(mid_x, mid_y) {
                    println!(
                        "  Center pixel ({},{}): RGB({}, {}, {})",
                        mid_x, mid_y, r, g, b
                    );
                }
            }

            // Calculate average color
            let total_pixels = (image.width() * image.height()) as f64;
            let data = image.data();
            let mut sum_r = 0u64;
            let mut sum_g = 0u64;
            let mut sum_b = 0u64;

            for chunk in data.chunks_exact(3) {
                sum_r += chunk[0] as u64;
                sum_g += chunk[1] as u64;
                sum_b += chunk[2] as u64;
            }

            let avg_r = (sum_r as f64 / total_pixels) as u8;
            let avg_g = (sum_g as f64 / total_pixels) as u8;
            let avg_b = (sum_b as f64 / total_pixels) as u8;

            println!("\n  Average color: RGB({}, {}, {})", avg_r, avg_g, avg_b);
        }
        Err(e) => {
            eprintln!("✗ Error reading JPEG file: {}", e);
            std::process::exit(1);
        }
    }
}
