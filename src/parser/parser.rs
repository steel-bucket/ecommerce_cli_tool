use clap::{Arg, Command, value_parser};
use serde::{Deserialize, Serialize};
use std::io::{stdin, stdout, Write};

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
 _______  _______  _        _______  _______ _________ _______  ______  
(  ____ \(  ___  )( \      (  ___  )(  ____ )\__   __/(  ____ \(  __  \ 
| (    \/| (   ) || (      | (   ) || (    )|   ) (   | (    \/| (  \  )
| (_____ | (___) || |      | (___) || (____)|   | |   | (__    | |   ) |
(_____  )|  ___  || |      |  ___  ||     __)   | |   |  __)   | |   | |
      ) || (   ) || |      | (   ) || (\ (      | |   | (      | |   ) |
/\____) || )   ( || (____/\| )   ( || ) \ \_____) (___| (____/\| (__/  )
\_______)|/     \|(_______/|/     \||/   \__/\_______/(_______/(______/                             
Welcome to the Ecommerce Website CLI Configurator!
"#
    );

    // Prompt for company name
    print!("Enter company name: ");
    stdout().flush().unwrap();
    let mut company_name = String::new();
    stdin().read_line(&mut company_name).unwrap();
    company_name = company_name.trim().to_string();

    // Prompt for address banner
    print!("Enter address banner: ");
    stdout().flush().unwrap();
    let mut address_banner = String::new();
    stdin().read_line(&mut address_banner).unwrap();
    address_banner = address_banner.trim().to_string();

    // Prompt for product categories
    print!("Enter product categories (comma-separated): ");
    stdout().flush().unwrap();
    let mut categories_str = String::new();
    stdin().read_line(&mut categories_str).unwrap();
    categories_str = categories_str.trim().to_string();

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