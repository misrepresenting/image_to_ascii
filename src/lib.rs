use std::error::Error;
use image::{GenericImageView, DynamicImage};
use image::io::Reader;

const CHARACTER_RAMP: &[u8] = 
    br#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;

pub fn build_image(
    mut args: impl Iterator<Item = String>
) -> Result<DynamicImage, Box<dyn Error>> {
    args.next();

    let file_name = args.next()
        .ok_or("file name not provided")?;
    let img = 
        Reader::open(file_name)?.decode()?;

    Ok(img)
}

fn pixel_to_char(rgba: image::Rgba<u8>) -> char {
    let image::Rgba([r, g, b, _]) = rgba;
    // 0 .. 255
    let intensity = 
        0.21 * r as f32 +
        0.72 * g as f32 +
        0.07 * b as f32;

    let index = ((intensity / 255.0) * (CHARACTER_RAMP.len() - 1) as f32).floor() as usize;
    let char = CHARACTER_RAMP[index] as char;

    char
}

pub fn run(img: DynamicImage) -> Result<(), Box<dyn Error>> {
    let (width, height) = img.dimensions();
    let mut buffer = String::with_capacity((width * height + height) as usize);

    for (x, y, color) in img.pixels() {
        let char = pixel_to_char(color);

        buffer.push(char);

        if x == width - 1 && y != height - 1 {
            buffer.push('\n');
        }
    }

    println!("{buffer}");

    Ok(())
}