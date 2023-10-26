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
        config.right_margin,
        config.threshold
    );
}

fn generate_image_from_args(args: &[String]) {
    if !validate_args(&args) {
        eprintln!("Usage: bitmap_type_tracer <path_to_font_image> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]");
        return;
    }

    let (font_path, sequence, text, chars_per_row) = parse_mandatory_args(&args);
    let (top_margin, bottom_margin, left_margin, right_margin, threshold) = parse_optional_args(&args);

    if args.contains(&"--save-json".to_string()) {
        let config = FontConfig {
            font_name: font_path.to_string(),
            sequence: sequence.to_string(),
            chars_per_row,
            top_margin,
            bottom_margin,
            left_margin,
            right_margin,
            threshold
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
        right_margin,
        threshold
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
    let chars_per_row = match args[4].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("You need to provide a valid number of characters per row.");
            std::process::exit(1);
        }
    };
    (&args[1], &args[2], &args[3], chars_per_row)
}

fn parse_optional_args(args: &[String]) -> (u32, u32, u32, u32, u8) {
    let mut top_margin = 0;
    let mut bottom_margin = 0;
    let mut left_margin = 0;
    let mut right_margin = 0;
    let mut threshold = 0;

    for i in 5..args.len() {
        match args[i].as_str() {
            "--top" => top_margin = parse_argument(&args[i+1],"Failed to read the top margin argument. Please provide a valid value." ),
            "--bottom" => bottom_margin = parse_argument(&args[i+1],"Failed to read the top margin argument. Please provide a valid value."),
            "--left" => left_margin = parse_argument(&args[i+1],"Failed to read the left margin argument. Please provide a valid value." ),
            "--right" => right_margin = parse_argument(&args[i+1],"Failed to read the right margin argument. Please provide a valid value." ),
            "--threshold" => threshold = parse_argument(&args[i+1],"Failed to read the threshold argument. Please provide a valid value." ),
            _ => {}
        }
    }

    (top_margin, bottom_margin, left_margin, right_margin, threshold as u8)
}

fn parse_argument(value: &str, error_message: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{}", error_message); // print the error to the stderr
            std::process::exit(1); // exit the process with a non-zero exit code
        }
    }
}
