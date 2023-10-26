use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::fs::write;

#[derive(Serialize, Deserialize)]
pub struct FontConfig {
    pub font_name: String,
    pub sequence: String,
    pub chars_per_row: u32,
    pub top_margin: u32,
    pub bottom_margin: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub threshold: u8
}

pub fn load_font_config(font_path: &str) -> FontConfig {
    let config_name = font_path.strip_suffix(".png").expect("Invalid font path").to_owned() + ".json";
    let mut file = File::open(config_name).expect("Failed to open font config");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read font config");

    serde_json::from_str(&contents).expect("Failed to parse font config")
}

pub fn save_font_config(font_path: &str, config: &FontConfig) {
    let config_name = font_path.strip_suffix(".png").expect("Invalid font path").to_owned() + ".json";

    // Use to_string_pretty for a prettified JSON format.
    let json_content = serde_json::to_string_pretty(&config).expect("Failed to serialize font config");

    write(&config_name, json_content).expect("Failed to save font config");
}
