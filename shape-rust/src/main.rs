extern crate image;

use image::{
    GenericImageView,
    Rgba,
    DynamicImage,
    Pixel
};
use imageproc::drawing::{
    draw_filled_rect_mut
};
use imageproc::rect::Rect;

fn main() {
    // Use the open function to load an image from a Path.
    // open returns a DynamicImage on  success.
    let mut img = image::open("src/input_image.jpeg").unwrap();

    // Define colors with rgba()
    let _red   = Rgba([255u8, 0u8,   0u8, 1u8]);
    let _green = Rgba([0u8,   255u8, 0u8, 1u8]);
    let _blue  = Rgba([0u8,   0u8,   255u8, 1u8]);
    let white = Rgba([255u8, 255u8, 255u8, 1u8]);

    // Draw rect on image
    draw_filled_rect_mut(&mut img, Rect::at(130, 10).of_size(2000, 2000), white);

    // Use mse() func to get difference
    mse_image(&img);

    // Write the contents of this image to the Writer in PNG format.
    img.save("src/test.png").unwrap();
}

fn mse_image(img: &DynamicImage) {
    let (width, height) = img.dimensions();

    println!("{width} X {height}, total: {} pixels", width*height);
    
    for x in 0 .. width {
        for y in 0 .. height {
            let px = img.get_pixel(x, y);
            let rgb = px.to_rgb();
            println!("RGB: {} {} {}", rgb[0], rgb[1], rgb[2]);
        }
    }
}
