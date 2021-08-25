use std::fs::File;
use std::io::Write;

use clap::{App, Arg};
use image::io::Reader as ImageReader;
use image::{DynamicImage, Rgb, RgbImage};

mod utils;

fn main() -> std::io::Result<()> {
    let matches = App::new("dot-image")
        .version("0.1.0")
        .author("N.H Nam <nguyenhoangnam.dev@gmail.com>")
        .about("Change image to dot")
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("image")
                .help("Import image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Store output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Set width of output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Set height of output image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .short("t")
                .long("threshold")
                .help("Threshold image")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("invert")
                .short("I")
                .long("invert")
                .help("Invert output"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Pring color dot"),
        )
        .get_matches();

    let image_path = match matches.value_of("image") {
        Some(path) => path,
        None => panic!("Missing image'spath"),
    };
    let width = match matches.value_of("width") {
        Some(w) => w.to_string().parse::<u32>().unwrap(),
        None => 0,
    };
    let height = match matches.value_of("height") {
        Some(h) => h.to_string().parse::<u32>().unwrap(),
        None => 0,
    };

    let output = matches.value_of("output").unwrap_or("");

    let invert = match matches.occurrences_of("invert") {
        0 => 0,
        _ => 1,
    };

    let color = match matches.occurrences_of("color") {
        0 => false,
        _ => true,
    };

    let threshold = matches
        .value_of("threshold")
        .unwrap_or("128")
        .parse::<u8>()
        .unwrap();

    let image_path_components = image_path.split(".").collect::<Vec<&str>>();
    let file_type = image_path_components[image_path_components.len() - 1];

    if color {
        let image = match ImageReader::open(image_path) {
            Ok(img) => img.decode().unwrap(),
            Err(_) => panic!("Can not open image"),
        };

        let result = utils::print_color_dot(image, width, height);

        if output == "" {
            println!("\n{}", result)
        } else {
            let mut file = File::create(output)?;
            file.write_all(result.as_bytes())?;
        }
    } else {
        if file_type == "gif" {
            let input = File::open(image_path).unwrap();

            let mut options = gif::DecodeOptions::new();
            options.set_color_output(gif::ColorOutput::RGBA);

            print!("\x1B[2J\x1B[1;1H");

            let mut decoder = options.read_info(input).unwrap();
            while let Some(frame) = decoder.read_next_frame().unwrap() {
                let mut x = 0;
                let mut y = 0;

                let mut img = RgbImage::new(frame.width as u32, frame.height as u32);
                let mut i = 0;
                while i < frame.buffer.len() {
                    img.put_pixel(
                        x,
                        y,
                        Rgb([frame.buffer[i], frame.buffer[i + 1], frame.buffer[i + 2]]),
                    );
                    x += 1;
                    if x == frame.width as u32 {
                        x = 0;
                        y += 1;
                    }
                    i += 4;
                }

                let image = DynamicImage::ImageRgb8(img);

                let result = utils::draw(image, width, height, invert, threshold);

                if output == "" {
                    println!("\n{}", result)
                } else {
                    let mut file = File::create(output)?;
                    file.write_all(result.as_bytes())?;
                }

                print!("\x1B[2J\x1B[1;1H");
            }
        } else {
            let image = match ImageReader::open(image_path) {
                Ok(img) => img.decode().unwrap(),
                Err(_) => panic!("Can not open image"),
            };

            let result = utils::draw(image, width, height, invert, threshold);

            if output == "" {
                println!("\n{}", result)
            } else {
                let mut file = File::create(output)?;
                file.write_all(result.as_bytes())?;
            }
        }
    }

    Ok(())
}
