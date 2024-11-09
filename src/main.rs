use indicatif::{ProgressBar, ProgressStyle};

pub mod vector;

fn main() {

    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    // This sets up format information for the .ppm file.
    println!("P3\n {} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Create a progress bar
    let progress_bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    let progress_bar_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/len").expect("Failed to set up progress bar")
        .progress_chars("#>-");
    progress_bar.set_style(progress_bar_style);

    // This puts the image out to the .ppm file.
    for j in 0..IMAGE_HEIGHT {
        
        progress_bar.inc(1);

        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
