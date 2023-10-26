extern crate image;
extern crate serde_derive;

use std::env;
mod font_config;
mod image_processing;

use font_config::{FontConfig, load_font_config, save_font_config};
use image_processing::generate_image;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {  // If only font and text are provided
        generate_image_from_config(&args)
    } else {
        generate_image_from_args(&args);
    }
}

fn generate_image_from_config(args: &[String]) {
    let font_path = &args[1];
    let text = &args[2];

    let config = load_font_config(font_path);

    generate_image(
        font_path,
        &config.sequence,
        text,
        config.chars_per_row,
        config.top_margin,
        config.bottom_margin,
        config.left_margin,
        config.right_margin
    );
}

fn generate_image_from_args(args: &[String]) {
    if !validate_args(&args) {
        eprintln!("Usage: bitmap_font_tool <path_to_font_image> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE]  [--save-json]");
        return;
    }

    let (font_path, sequence, text, chars_per_row) = parse_mandatory_args(&args);
    let (top_margin, bottom_margin, left_margin, right_margin) = parse_optional_args(&args);

    if args.contains(&"--save-json".to_string()) {
        let config = FontConfig {
            font_name: font_path.to_string(),
            sequence: sequence.to_string(),
            chars_per_row,
            top_margin,
            bottom_margin,
            left_margin,
            right_margin
        };
        save_font_config(font_path, &config);
    }

    generate_image(
        font_path,
        sequence,
        text,
        chars_per_row,
        top_margin,
        bottom_margin,
        left_margin,
        right_margin
    );
}

fn validate_args(args: &[String]) -> bool {
    // Base case: we always need at least 5 arguments.
    if args.len() < 5 {
        return false;
    }

    // Check if --save-json flag is present
    let save_json_flag = args.contains(&"--save-json".to_string());

    // Calculate the number of expected arguments excluding the flag
    let expected_args = if save_json_flag { args.len() - 1 } else { args.len() };

    expected_args <= 5 || (expected_args - 5) % 2 == 0
}

fn parse_mandatory_args(args: &[String]) -> (&String, &String, &String, u32) {
    (&args[1], &args[2], &args[3], args[4].parse().expect("Failed to parse chars_per_row"))
}

fn parse_optional_args(args: &[String]) -> (u32, u32, u32, u32) {
    let mut top_margin = 0;
    let mut bottom_margin = 0;
    let mut left_margin = 0;
    let mut right_margin = 0;

    for i in 5..args.len() {
        match args[i].as_str() {
            "--top" => top_margin = args[i+1].parse().expect("Failed to parse top margin"),
            "--bottom" => bottom_margin = args[i+1].parse().expect("Failed to parse bottom margin"),
            "--left" => left_margin = args[i+1].parse().expect("Failed to parse left margin"),
            "--right" => right_margin = args[i+1].parse().expect("Failed to parse right margin"),
            _ => {}
        }
    }

    (top_margin, bottom_margin, left_margin, right_margin)
}

