use std::io::{self,Write};

fn main() {
    // Image
    const image_width: i32 = 256;
    const image_height: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r:f64 = i as f64 / (image_width - 1) as f64;
            let g:f64 = j as f64 / (image_height - 1) as f64;
            let b:f64 = 0.25;

            let ir:i32 = (255.999 * r) as i32;
            let ig:i32 = (255.999 * g) as i32;
            let ib:i32 = (255.999 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprintln!("\nDone.\n");
}
