use image::GenericImageView;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

fn main() {
    let impath = "./assets/gojo.jpg";
    let img = image::open(impath)
        .expect("[error]: couldn't read image file");
    let (w, h) = img.dimensions();

    // Create a standard output stream
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.0;

            // Set the foreground color
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(rgb[0], rgb[1], rgb[2]))))
                .expect("Failed to set color");

            // Print the character
            write!(&mut stdout, "@").expect("Failed to write character");
        }

        // Reset the color and move to the next line
        stdout.reset().expect("Failed to reset color");
        writeln!(&mut stdout).expect("Failed to move to the next line");
    }
}
