use std::env;
mod font_config;
mod image_processing;
mod translations;

use font_config::{FontConfig, load_font_config, save_font_config};
use image_processing::generate_image;

use locale_config::Locale;
use once_cell::sync::Lazy;
use crate::translations::Translation;
use std::sync::Arc;

static TRANSLATION: Lazy<Arc<dyn Translation>> = Lazy::new(|| {
    let locale = Locale::user_default().to_string();
    let boxed_translation = translations::get_translation_for_locale(&locale);
    Arc::from(boxed_translation)
});


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] == "--help" {
        eprintln!("{}", TRANSLATION.full_help());
        return;
    }

    if args.len() < 3 || args[1] == "--version" {
        eprintln!("{}", TRANSLATION.version());
        return;
    }

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
        text,
        &config
    );
}

fn generate_image_from_args(args: &[String]) {
    if !validate_args(args) {
        eprintln!("{}", TRANSLATION.full_help());
        return;
    }

    let (font_path, sequence, text, chars_per_row) = parse_mandatory_args(args);
    let (top_margin, bottom_margin, left_margin, right_margin, threshold) = parse_optional_args(args);

    let config = FontConfig {
        sequence: sequence.to_string(),
        chars_per_row,
        top_margin,
        bottom_margin,
        left_margin,
        right_margin,
        threshold
    };

    if args.contains(&"--save-json".to_string()) {
        save_font_config(font_path, &config);
    }

    generate_image(
        font_path,
        text,
        &config
    );
}

fn validate_args(args: &[String]) -> bool {
    // Base case: we always need at least 5 arguments.
    if args.len() < 5 {
        return false;
    }

    let save_json_flag = args.contains(&"--save-json".to_string());

    // Calculate the number of expected arguments excluding the flag
    let expected_args = if save_json_flag {
        args.len() - 1
    } else {
        args.len()
    };

    expected_args <= 5 || (expected_args - 5) % 2 == 0
}

fn parse_mandatory_args(args: &[String]) -> (&String, &String, &String, u32) {
    let chars_per_row = match args[4].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{}", TRANSLATION.err_invalid_num_of_chars());
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
            "--top" => top_margin = parse_argument(&args[i+1],TRANSLATION.err_invalid_top_margin()),
            "--bottom" => bottom_margin = parse_argument(&args[i+1],TRANSLATION.err_invalid_bottom_margin()),
            "--left" => left_margin = parse_argument(&args[i+1],TRANSLATION.err_invalid_left_margin()),
            "--right" => right_margin = parse_argument(&args[i+1],TRANSLATION.err_invalid_right_margin()),
            "--threshold" => threshold = parse_argument(&args[i+1],TRANSLATION.err_invalid_threshold()),
            _ => {}
        }
    }

    (top_margin, bottom_margin, left_margin, right_margin, threshold as u8)
}

fn parse_argument(value: &str, error_message: &str) -> u32 {
    match value.parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
    }
}
