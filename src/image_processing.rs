use image::{ImageBuffer, GenericImageView, Rgba};
use crate::TRANSLATION;

pub fn generate_image(
    font_path: &str,
    sequence: &str,
    text: &str,
    chars_per_row: u32,
    top_margin: u32,
    bottom_margin: u32,
    left_margin: u32,
    right_margin: u32,
    threshold: u8
) {
    let font_image = load_font_image(font_path);
    let (char_width, char_height) = calculate_character_dimensions(&font_image, sequence, chars_per_row, top_margin, bottom_margin, left_margin, right_margin);

    let output_width = char_width * text.len() as u32;
    let mut output_image = ImageBuffer::new(output_width, char_height);

    for (idx, character) in text.chars().enumerate() {
        let pos = find_character_position(character, sequence);

        if let Some(position) = pos {
            copy_character_to_output(
                &font_image,
                &mut output_image,
                position,
                char_width,
                char_height,
                chars_per_row,
                left_margin,
                top_margin, idx
            );
        } else if None == pos {
            eprintln!("{}", TRANSLATION.character_not_found(character));
            fill_with_bg_color(&mut output_image, idx as u32 * char_width, 0, char_width, char_height);
        }
    }
    if threshold > 0 {
        remove_background(&mut output_image, threshold);
    }

    output_image.save("output.png").expect(TRANSLATION.err_failed_to_save_output_image())
}

fn load_font_image(font_path: &str) -> image::DynamicImage {
    image::open(font_path).expect(TRANSLATION.err_failed_to_open_font_image())
}

fn calculate_character_dimensions(
    font_image: &image::DynamicImage,
    sequence: &str,
    chars_per_row: u32,
    top_margin: u32,
    bottom_margin: u32,
    left_margin: u32,
    right_margin: u32
) -> (u32, u32) {
    let char_width = (font_image.width() - (left_margin + right_margin)) / chars_per_row;
    let char_height = (font_image.height() - (top_margin + bottom_margin)) / (sequence.len() as u32 / chars_per_row);
    (char_width, char_height)
}

fn find_character_position(character: char, sequence: &str) -> Option<usize> {
    let pos = sequence.find(character);
    if pos.is_none() && character.is_ascii_lowercase() {
        return sequence.find(character.to_ascii_uppercase());
    }
    pos
}

fn copy_character_to_output(
    font_image: &image::DynamicImage,
    output_image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    position: usize,
    char_width: u32,
    char_height: u32,
    chars_per_row: u32,
    left_margin: u32,
    top_margin: u32,
    idx: usize
) {
    let x_offset = left_margin + (position as u32 % chars_per_row) * char_width;
    let y_offset = top_margin + (position as u32 / chars_per_row) * char_height;
    let section = font_image.view(x_offset, y_offset, char_width, char_height);
    image::imageops::replace(output_image, &section, idx as u32 * char_width, 0);
}

fn estimate_bg_color(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Rgba<u8> {
    let (width, height) = image.dimensions();

    // Sample the corners for background color estimation
    let tl = *image.get_pixel(0, 0);
    let tr = *image.get_pixel(width - 1, 0);
    let bl = *image.get_pixel(0, height - 1);
    let br = *image.get_pixel(width - 1, height - 1);

    // Take an average of the corner pixels to estimate the background color
    Rgba([
        ((tl[0] as u32 + tr[0] as u32 + bl[0] as u32 + br[0] as u32) / 4) as u8,
        ((tl[1] as u32 + tr[1] as u32 + bl[1] as u32 + br[1] as u32) / 4) as u8,
        ((tl[2] as u32 + tr[2] as u32 + bl[2] as u32 + br[2] as u32) / 4) as u8,
        255, // Alpha channel
    ])
}

fn fill_with_bg_color(
    output_image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32
) {
    let bg_color = estimate_bg_color(output_image);
    for x in start_x..(start_x + width) {
        for y in start_y..(start_y + height) {
            output_image.put_pixel(x, y, bg_color);
        }
    }
}

fn remove_background(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, threshold: u8) {
    let bg_color = estimate_bg_color(image);
    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = *image.get_pixel(x, y);
            if is_close_to(pixel, bg_color, threshold) {
                image.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], 0]));  // Make it transparent
            }
        }
    }
}

fn is_close_to(a: Rgba<u8>, b: Rgba<u8>, threshold: u8) -> bool {
    (a[0] as i32 - b[0] as i32).abs() <= threshold as i32 &&
        (a[1] as i32 - b[1] as i32).abs() <= threshold as i32 &&
        (a[2] as i32 - b[2] as i32).abs() <= threshold as i32
}