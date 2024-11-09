use crate::vector::Vec3;

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0, 1] component values to the byte range [0, 255].
    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    // Write out the pixel color components
    println!("{} {} {}", rbyte, gbyte, bbyte);
}
