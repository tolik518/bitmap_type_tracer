pub struct English;

impl super::Translation for English {
    fn help(&self) -> &'static str {
        "Usage: \n \
                            If your font already has a configuration file: \n \
                            bitmap_type_tracer <path_to_bitmap_font> <text>\n\n \
                            If you want to generate a configuration file: \n \
                            bitmap_type_tracer <path_to_bitmap_font> <sequence> <text> <chars_per_row> [--top VALUE] [--bottom VALUE] [--left VALUE] [--right VALUE] [--threshold VALUE] [--save-json]\n   \
                            path_to_bitmap_font     literally the path to the bitmap.png containing the characters\n   \
                            sequence                the sequence of characters that you see in the bitmap font e.x 'ABCDEF...'\n   \
                            text                    the text you want to write with the provided bitmap font\n   \
                            chars_per_row           how many characters are in a row in the provided bitmap font\n \
                            Margins: \n   \
                            --top VALUE             the number of pixels to crop from the top of the image\n   \
                            --bottom VALUE          the number of pixels to crop from the bottom of the image\n   \
                            --left VALUE            the number of pixels to crop from the left of the image\n   \
                            --right VALUE           the number of pixels to crop from the right of the image\n \
                            Other options: \n   \
                            --threshold VALUE       the value to determine the threshold for making the background transparent (0-255)\n   \
                            --save-json             save the configuration to a json file\n   \
                            --help                  print this help message\n   \
                            --version               print the version of the program\n\n \
                            For example usage check out the README.md in the repository \n   \
                            "
    }
    fn version(&self) -> &'static str {env!("CARGO_PKG_VERSION")}
    fn repository(&self) -> &'static str {env!("CARGO_PKG_REPOSITORY")}
    fn name(&self) -> &'static str {env!("CARGO_PKG_NAME")}
    fn author(&self) -> &'static str {env!("CARGO_PKG_AUTHORS")}
    fn full_help(&self) -> String {
        format!(
            "{} by {}\nVersion: {}\nRepository: {}\n{}",
            self.name(), self.author(), self.version(), self.repository(), self.help()
        )
    }
    fn character_not_found(&self, character: char) -> String {
        format!("Character '{}' not found in sequence. Trying to use the background-color instead.", character)
    }

    fn err_invalid_num_of_chars(&self) -> &'static str {"You need to provide a valid number of characters per row."}
    fn err_invalid_threshold(&self) -> &'static str {"You need to provide a valid threshold value (0-255)."}
    fn err_invalid_left_margin(&self) -> &'static str {"Failed to read the right margin argument. Please provide a valid value."}
    fn err_invalid_right_margin(&self) -> &'static str {"Failed to read the right margin argument. Please provide a valid value."}
    fn err_invalid_top_margin(&self) -> &'static str {"Failed to read the top margin argument. Please provide a valid value."}
    fn err_invalid_bottom_margin(&self) -> &'static str {"Failed to read the bottom margin argument. Please provide a valid value."}

    fn err_failed_to_read_config(&self) -> &'static str {"Failed to read the font config (json) file."}
    fn err_failed_to_parse_config(&self) -> &'static str {"Failed to parse the font config (json) file."}
    fn err_failed_to_open_config(&self) -> &'static str {"Failed to open the font config (json) file."}
    fn err_failed_to_serialize_config(&self) -> &'static str {"Failed to serialize the font config (json) file."}
    fn err_failed_to_save_config(&self) -> &'static str {"Failed to save the font config (json) file."}
    fn err_invalid_font_path(&self) -> &'static str {"Invalid font path"}

    fn err_failed_to_save_output_image(&self) -> &'static str {"Failed to save output image"}
    fn err_failed_to_open_font_image(&self) -> &'static str {"Failed to open font image"}
}