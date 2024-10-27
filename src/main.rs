fn main() {
    
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    // This sets up format information for the .ppm file.
    println!("P3\n {} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);


    // This puts the image out to the .ppm file.
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
