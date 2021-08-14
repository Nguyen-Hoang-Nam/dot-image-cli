use clap::{App, Arg};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::Rgb;
// use image::{GenericImage, GenericImageView};

static FIRST_EMOJI: u32 = 10240;

fn initial_row(row: &mut Vec<u8>, row_width: u32) {
    for i in 0..row_width {
        row[i as usize] = 0;
    }
}

fn utf32_to_utf8(utf32: u32) -> Vec<u8> {
    let mut utf8 = vec![];

    if utf32 <= 0x7F {
        utf8.push(utf32 as u8);
        utf8.push(0);
        utf8.push(0);
        utf8.push(0);
    } else if utf32 <= 0x7FF {
        utf8.push((0xC0 | (utf32 >> 6)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
        utf8.push(0);
        utf8.push(0);
    } else if utf32 <= 0xFFFF {
        utf8.push((0xE0 | (utf32 >> 12)) as u8);
        utf8.push((0x80 | ((utf32 >> 6) & 0x3F)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
        utf8.push(0);
    } else if utf32 <= 0x10FFFF {
        utf8.push((0xF0 | (utf32 >> 18)) as u8);
        utf8.push((0x80 | ((utf32 >> 12) & 0x3F)) as u8);
        utf8.push((0x80 | ((utf32 >> 6) & 0x3F)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
    }

    utf8
}

fn from_utf32(utf32: u32) -> String {
    let utf8 = utf32_to_utf8(utf32);
    std::str::from_utf8(&utf8).unwrap().to_string()
}

fn main() {
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
            Arg::with_name("invert")
                .short("I")
                .long("invert")
                .help("Invert output"),
        )
        .get_matches();

    let image_path = match matches.value_of("image") {
        Some(path) => path,
        None => panic!("Missing image'spath"),
    };
    let width = match matches.value_of("width") {
        Some(w) => w.to_string().parse::<u32>().unwrap(),
        None => 20,
    };
    let height = match matches.value_of("height") {
        Some(h) => h.to_string().parse::<u32>().unwrap(),
        None => 20,
    };

    let invert = match matches.occurrences_of("invert") {
        0 => 0,
        _ => 1,
    };

    let image = match ImageReader::open(image_path) {
        Ok(img) => img.decode().unwrap(),
        Err(_) => panic!("Can not open image"),
    };
    let scale = image.resize(width, height, FilterType::Triangle);

    let mut grayscale_rgb = scale.grayscale().to_rgb8();

    for x in 0..width {
        for y in 0..height {
            if grayscale_rgb.get_pixel(x, y)[0] < 128 {
                grayscale_rgb.put_pixel(x, y, Rgb([invert, invert, invert]))
            } else {
                grayscale_rgb.put_pixel(x, y, Rgb([1 - invert, 1 - invert, 1 - invert]))
            }
        }
    }

    let mut row: Vec<u8> = Vec::new();

    let row_width = (width + width % 2) / 2;
    for _ in 0..row_width {
        &row.push(0);
    }

    for y in 0..height {
        let x0;
        let x1;

        if y % 4 == 0 {
            initial_row(&mut row, row_width);

            x0 = 1;
            x1 = 8;
        } else if y % 4 == 1 {
            x0 = 2;
            x1 = 16;
        } else if y % 4 == 2 {
            x0 = 4;
            x1 = 32;
        } else {
            x0 = 64;
            x1 = 128;
        }

        for x in (0..width).step_by(2) {
            let index = (x - x % 2) / 2;

            if x + 1 <= width - 1 {
                row[index as usize] += grayscale_rgb.get_pixel(x, y)[0] * x0
                    + grayscale_rgb.get_pixel(x + 1, y)[0] * x1;
            }
        }

        if y % 4 == 3 || y == height - 1 {
            let mut icon_list: String = "".to_owned();

            for i in 0..row_width {
                icon_list.push_str(&from_utf32(row[i as usize] as u32 + FIRST_EMOJI));
            }
            println!("{}", icon_list)
        }
    }
}
