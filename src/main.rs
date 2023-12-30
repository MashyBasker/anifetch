use image::GenericImageView;
use colored::Colorize;

fn main() {
    let impath = "../assets/gojo.jpg";
    let img = image::open(impath)
        .expect("[error]: couldn't read image file");
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.0;
            print!("{}", "@".truecolor(rgb[0], rgb[1], rgb[2]));
        }
        print!("\n");
    }
}
