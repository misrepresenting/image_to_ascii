//! #image_to_ascii
//! 
//! cli program that turns images into ascii characters

use std::env;
use std::process;
use image_to_ascii::{self, build_image};

fn main() {
    let img = build_image(env::args()).unwrap_or_else(|e| {
        eprintln!("error with args: {e}");
        process::exit(1);
    });

    if let Err(e) = image_to_ascii::run(img) {
        eprintln!("error with app: {e}");
    }
}

