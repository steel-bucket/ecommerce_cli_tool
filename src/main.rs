// src/main.rs
pub mod parser;
use std::fs::File;
use std::io::Write;
use parser::parser::parse_args;
use parser::choose::{choose_files, write_web_config};
use serde_json;

fn main() {
    // 1. Parse CLI parameters using clap (in parser.rs)
    let config = parse_args();

    // 2. Write the basic configuration to config.json
    let config_json = serde_json::to_string_pretty(&config)
        .expect("Failed to serialize configuration");
    let mut file = File::create("config.json")
        .expect("Unable to create config.json");
    file.write_all(config_json.as_bytes())
        .expect("Unable to write config.json");
    println!("config.json has been written.");

    // 3. Now call choose_files (in choose.rs) to prompt the user for file selections.
    let file_choices = choose_files(&config.product_categories);

    // 4. Generate webConfig.ts using the configuration and file choices.
    write_web_config(&config, &file_choices);
}
