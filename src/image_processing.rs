use image::{ImageBuffer, GenericImageView};

pub fn generate_image(
    font_path: &str,
    sequence: &str,
    text: &str,
    chars_per_row: u32,
    top_margin: u32,
    bottom_margin: u32,
    left_margin: u32,
    right_margin: u32
) {
    let font_image = load_font_image(font_path);
    let (char_width, char_height) = calculate_character_dimensions(&font_image, sequence, chars_per_row, top_margin, bottom_margin, left_margin, right_margin);

    let output_width = char_width * text.len() as u32;
    let mut output_image = ImageBuffer::new(output_width, char_height);

    for (idx, character) in text.chars().enumerate() {
        let pos = find_character_position(character, sequence);

        if let Some(position) = pos {
            copy_character_to_output(&font_image, &mut output_image, position, char_width, char_height, chars_per_row, left_margin, top_margin, idx);
        } else {
            eprintln!("Character '{}' not found in sequence", character);
        }
    }

    output_image.save("output.png").expect("Failed to save output image");
}

fn load_font_image(font_path: &str) -> image::DynamicImage {
    image::open(font_path).expect("Failed to open font image")
}

fn calculate_character_dimensions(font_image: &image::DynamicImage, sequence: &str, chars_per_row: u32, top_margin: u32, bottom_margin: u32, left_margin: u32, right_margin: u32) -> (u32, u32) {
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

fn copy_character_to_output(font_image: &image::DynamicImage, output_image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>, position: usize, char_width: u32, char_height: u32, chars_per_row: u32, left_margin: u32, top_margin: u32, idx: usize) {
    let x_offset = left_margin + (position as u32 % chars_per_row) * char_width;
    let y_offset = top_margin + (position as u32 / chars_per_row) * char_height;
    let section = font_image.view(x_offset, y_offset, char_width, char_height);
    image::imageops::replace(output_image, &section, idx as u32 * char_width, 0);
}
