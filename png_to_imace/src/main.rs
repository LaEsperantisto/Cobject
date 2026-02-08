use image::GenericImageView;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().collect::<Vec<_>>().len() != 3 {
        panic!("There should be exactly 2 arguments!");
    }
    // Path to input image
    let input_path = &env::args().collect::<Vec<_>>()[1]; // or "input.bmp"
    let output_path = &env::args().collect::<Vec<_>>()[2];

    // Load the image
    let img = image::open(&Path::new(&input_path))?;
    let (width, height) = img.dimensions();

    let mut file = File::create(&output_path)?;

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).0; // [R,G,B,A] or [R,G,B]
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            // Convert to hex string
            let hex = format!("{:02X}{:02X}{:02X}", r, g, b);

            write!(file, "{} ", hex)?;
        }
        writeln!(file, "EOL")?;
    }

    println!("Image converted successfully to '{}'", output_path);
    Ok(())
}
