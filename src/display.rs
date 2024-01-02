use image::GenericImageView;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::Write;
use std::fs;
use rand::Rng;

// TODO: fix dependency on .png format 
fn choose_image() -> Option<String> {
    let path = "./images";
    let c = fs::read_dir(path).unwrap().count();
    if c == 0 {
        None
    } else {
        let img_idx = rand::thread_rng().gen_range(0..c);
        Some(format!("./images/{}.png", img_idx))
    }
}


pub fn convert() {
    let impath = choose_image().unwrap();
    // println!("{}", impath);
    let img = image::open(impath)
        .expect("[error]: couldn't read image file");
    let ascii_width = 40;
    let ascii_height = 30;

    // Calculate the scaling factors
    let scale_x = img.width() as f32 / ascii_width as f32;
    let scale_y = img.height() as f32 / ascii_height as f32;
    // Create a standard output stream
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    for y in 0..ascii_height {
        for x in 0..ascii_width {
            let orig_x = (x as f32 * scale_x).round() as u32;
            let orig_y = (y as f32 * scale_y).round() as u32;
            let pixel = img.get_pixel(orig_x, orig_y);
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