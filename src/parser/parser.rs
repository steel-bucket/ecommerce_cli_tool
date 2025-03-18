// src/parser.rs
use clap::{Arg, Command, value_parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub company_name: String,
    pub address_banner: String,
    pub product_categories: Vec<String>,
}

/// Parses CLI arguments and returns a Config struct.
pub fn parse_args() -> Config {
    // Print ASCII art welcome text:
    println!(
        r#"
  ____                            _         ____ _     ___ 
 |  _ \ ___  ___ ___  _ __   ___ | |_ ___  / ___| |   |_ _|
 | |_) / _ \/ __/ _ \| '_ \ / _ \| __/ _ \| |   | |    | | 
 |  _ <  __/ (_| (_) | |_) | (_) | ||  __/| |___| |___ | | 
 |_| \_\___|\___\___/| .__/ \___/ \__\___| \____|_____|___|
                     |_|                                   
Welcome to the Ecommerce Website CLI Configurator!
"#
    );

    let matches = Command::new("Ecommerce CLI Configurator")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Configures an ecommerce website")
        .arg(
            Arg::new("company")
                .short('n')
                .long("company")
                .value_name("COMPANY_NAME")
                .help("Sets the company name")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS_BANNER")
                .help("Sets the address and banner text")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("categories")
                .short('c')
                .long("categories")
                .value_name("PRODUCT_CATEGORIES")
                .help("Comma-separated list of product categories")
                .required(true)
                .num_args(1)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let company_name = matches.get_one::<String>("company").unwrap().to_string();
    let address_banner = matches.get_one::<String>("address").unwrap().to_string();
    let categories_str = matches.get_one::<String>("categories").unwrap();
    let product_categories: Vec<String> = categories_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Config {
        company_name,
        address_banner,
        product_categories,
    }
}
