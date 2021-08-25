use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgb};

static FIRST_EMOJI: u32 = 10240;

fn initial_row(row: &mut Vec<u8>, row_width: u32) {
    for i in 0..row_width {
        row[i as usize] = 0;
    }
}

fn initial_row_32(row: &mut Vec<u32>, row_width: u32) {
    for i in 0..row_width {
        row[i as usize] = 0;
    }
}

pub fn utf32_to_utf8(utf32: u32) -> Vec<u8> {
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

pub fn draw(
    image: DynamicImage,
    mut width: u32,
    mut height: u32,
    invert: u8,
    threshold: u8,
) -> String {
    if height == 0 {
        let (current_width, current_height) = image.dimensions();

        height = width * current_height / current_width;
    }

    if width == 0 {
        let (current_width, current_height) = image.dimensions();

        width = height * current_width / current_height;
    }

    let scale = image.resize_exact(width, height, FilterType::Triangle);

    let mut scale_rgb = scale.grayscale().to_rgb8();

    for x in 0..width {
        for y in 0..height {
            if scale_rgb.get_pixel(x, y)[0] < threshold {
                scale_rgb.put_pixel(x, y, Rgb([invert, invert, invert]))
            } else {
                scale_rgb.put_pixel(x, y, Rgb([1 - invert, 1 - invert, 1 - invert]))
            }
        }
    }

    let mut row: Vec<u8> = Vec::new();

    let mut result: String = "".to_owned();

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
                row[index as usize] +=
                    scale_rgb.get_pixel(x, y)[0] * x0 + scale_rgb.get_pixel(x + 1, y)[0] * x1;
            }
        }

        if y % 4 == 3 || y == height - 1 {
            let mut icon_list: String = "".to_owned();

            for i in 0..row_width {
                icon_list.push_str(&from_utf32(row[i as usize] as u32 + FIRST_EMOJI));
            }
            result.push_str(&icon_list);
            result.push_str("\n");
        }
    }

    return result;
}

pub fn print_color_dot(image: DynamicImage, mut width: u32, mut height: u32) -> String {
    if height == 0 {
        let (current_width, current_height) = image.dimensions();

        height = width * current_height / current_width;
    }

    if width == 0 {
        let (current_width, current_height) = image.dimensions();

        width = height * current_width / current_height;
    }

    let scale = image.resize_exact(width, height, FilterType::Triangle);

    let scale_rgb = scale.to_rgb8();

    let mut red: Vec<u32> = Vec::new();
    let mut green: Vec<u32> = Vec::new();
    let mut blue: Vec<u32> = Vec::new();

    let mut result: String = "".to_owned();

    let row_width = (width + width % 2) / 2;
    for _ in 0..row_width {
        &red.push(0);
        &green.push(0);
        &blue.push(0);
    }

    for y in 0..height {
        if y % 4 == 0 {
            initial_row_32(&mut red, row_width);
            initial_row_32(&mut green, row_width);
            initial_row_32(&mut blue, row_width);
        }

        for x in (0..width).step_by(2) {
            let index = (x - x % 2) / 2;

            if x + 1 <= width - 1 {
                red[index as usize] +=
                    scale_rgb.get_pixel(x, y)[0] as u32 + scale_rgb.get_pixel(x + 1, y)[0] as u32;
                green[index as usize] +=
                    scale_rgb.get_pixel(x, y)[1] as u32 + scale_rgb.get_pixel(x + 1, y)[1] as u32;
                blue[index as usize] +=
                    scale_rgb.get_pixel(x, y)[2] as u32 + scale_rgb.get_pixel(x + 1, y)[2] as u32;
            }
        }

        if y % 4 == 3 || y == height - 1 {
            let mut icon_list: String = "".to_owned();
            let t = 2 * (y % 4 + 1);

            for i in 0..row_width {
                let color_term = format!(
                    "\x1B[38;2;{};{};{}m",
                    (red[i as usize] / t) as u8,
                    (green[i as usize] / t) as u8,
                    (blue[i as usize] / t) as u8
                );
                icon_list.push_str(&color_term);
                icon_list.push_str(&from_utf32(10495));
            }

            result.push_str(&icon_list);
            result.push_str("\n");
        }
    }

    return result;
}
