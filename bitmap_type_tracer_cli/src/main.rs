// main.rs for bitmap_type_tracer_cli

pub use bitmap_type_tracer_lib::{generate_image_from_config, generate_image_from_args, LANG_OVERRIDE, TRANSLATION};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for --lang argument and set it globally
    let mut lang_override: Option<String> = None;

    // Search for --lang argument
    args.iter().enumerate().for_each(|(i, arg)| {
        if arg == "--lang" && i + 1 < args.len() {
            lang_override = Some(args[i + 1].clone());
        }
    });

    // Set the global LANG_OVERRIDE variable
    let _ = LANG_OVERRIDE.set(lang_override.clone());

    if args[1] == "--version" {
        println!("{}", TRANSLATION.version());
        return;
    }

    if args.len() < 3 || args[1] == "--help" {
        println!("{}", TRANSLATION.full_help());
        return;
    }

    // we have exactly 3 arguments if the user wants to generate from config
    // if additionally the language is set, then we have 5 arguments
    if args.len() == 3 || (args.len() == 5 && lang_override.is_some()) {
        generate_image_from_config(&args)
    } else {
        generate_image_from_args(&args);
    }
}