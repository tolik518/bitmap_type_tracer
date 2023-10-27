use crate::translations::en::English;
use crate::translations::kk::Kazakh;

mod en;
mod kk;

pub fn get_translation_for_locale(locale: &str) -> Box<dyn Translation> {
    match locale {
        "kk" => Box::new(Kazakh),
        "kz" => Box::new(Kazakh),
        // ... other locales
        _ => Box::new(English),
    }
}

pub trait Translation : Send + Sync  {
    fn help(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn repository(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn author(&self) -> &'static str;
    fn full_help(&self) -> String;
    fn character_not_found(&self, character: char) -> String;

    fn err_invalid_num_of_chars(&self) -> &'static str;
    fn err_invalid_threshold(&self) -> &'static str;
    fn err_invalid_left_margin(&self) -> &'static str;
    fn err_invalid_right_margin(&self) -> &'static str;
    fn err_invalid_top_margin(&self) -> &'static str;
    fn err_invalid_bottom_margin(&self) -> &'static str;

    fn err_failed_to_read_config(&self) -> &'static str;
    fn err_failed_to_parse_config(&self) -> &'static str;
    fn err_failed_to_open_config(&self) -> &'static str;
    fn err_failed_to_serialize_config(&self) -> &'static str;
    fn err_failed_to_save_config(&self) -> &'static str;
    fn err_invalid_font_path(&self) -> &'static str;

    fn err_failed_to_save_output_image(&self) -> &'static str;
    fn err_failed_to_open_font_image(&self) -> &'static str;
}


