#![allow(clippy::expect_fun_call)]
use crate::{TRANSLATION};

use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::fs::write;

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    pub sequence: String,
    pub chars_per_row: u32,
    pub top_margin: u32,
    pub bottom_margin: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub threshold: u8
}

pub fn load_font_config(font_path: &str) -> FontConfig {
    let config_name = font_path.strip_suffix(".png").expect(TRANSLATION.err_invalid_font_path()).to_owned() + ".json";
    let mut file = File::open(config_name).expect(TRANSLATION.err_failed_to_open_config());
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(TRANSLATION.err_failed_to_read_config());

    serde_json::from_str(&contents).expect(TRANSLATION.err_failed_to_parse_config())
}

pub fn save_font_config(font_path: &str, config: &FontConfig) {
    let config_name = font_path.strip_suffix(".png").expect(TRANSLATION.err_invalid_font_path()).to_owned() + ".json";

    let json_content = serde_json::to_string_pretty(&config).expect(TRANSLATION.err_failed_to_serialize_config());

    write(config_name, json_content).expect(TRANSLATION.err_failed_to_save_config())
}
