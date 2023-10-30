use image::GenericImageView;
use color_thief::{get_palette, ColorFormat};

// load an image from path as an argument
fn main(){
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let img = image::open(path).unwrap();
    println!("dimensions {:?}", img.dimensions());
    let bytes: &[u8] = img.as_bytes();

    // use color-thief to get the dominant colors
    let palette = get_palette(&bytes, ColorFormat::Argb, 9, 4).unwrap();

    // print the colors in hex
    for color in palette {
        println!("{:x}", color);
    }
}
