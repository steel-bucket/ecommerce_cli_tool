// src/choose.rs
use dialoguer::Input;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
/// Stores the selected files.
pub struct FileChoices {
    pub banner_file: String,
    pub category_images: HashMap<String, [String; 3]>,
}

/// Prompts the user to choose a banner file and, for each product category,
/// asks for three image files.
pub fn choose_files(categories: &[String]) -> FileChoices {
    // Ask for the banner poster file:
    let banner_file: String = Input::new()
        .with_prompt("Choose your banner poster file (enter file path)")
        .interact_text()
        .unwrap();

    let mut category_images = HashMap::new();
    // For each category, ask for 3 image sources:
    for category in categories {
        println!("\nFor category '{}':", category);
        let img1: String = Input::new()
            .with_prompt(format!("Choose image 1 for '{}'", category))
            .interact_text()
            .unwrap();
        let img2: String = Input::new()
            .with_prompt(format!("Choose image 2 for '{}'", category))
            .interact_text()
            .unwrap();
        let img3: String = Input::new()
            .with_prompt(format!("Choose image 3 for '{}'", category))
            .interact_text()
            .unwrap();

        category_images.insert(category.clone(), [img1, img2, img3]);
    }

    FileChoices {
        banner_file,
        category_images,
    }
}

/// Generates the webConfig.ts file from the provided configuration and file choices.
pub fn write_web_config(config: &crate::parser::parser::Config, choices: &FileChoices) {
    let mut output = String::new();
    output.push_str("export const PRODUCT_CATEGORIES = [\n");
    for category in &config.product_categories {
        // For simplicity, convert the category to a lowercase value without spaces.
        let value = category.to_lowercase().replace(" ", "");
        output.push_str("    {\n");
        output.push_str(&format!("        label: '{}',\n", category));
        output.push_str(&format!("        value: '{}' as const,\n", value));
        output.push_str("        featured: [\n");
        let images = choices.category_images.get(category).unwrap();
        output.push_str(&format!(
            "            {{ name: 'Editor picks', href: `/products?category={}`, imageSrc: '{}', }},\n",
            value, images[0]
        ));
        output.push_str(&format!(
            "            {{ name: 'New Arrivals', href: `/products?category={}&sort=desc`, imageSrc: '{}', }},\n",
            value, images[1]
        ));
        output.push_str(&format!(
            "            {{ name: 'Bestsellers', href: `/products?category={}`, imageSrc: '{}', }},\n",
            value, images[2]
        ));
        output.push_str("        ],\n");
        output.push_str("    },\n");
    }
    output.push_str("];\n");

    // Write to webConfig.ts
    let mut file = File::create("webConfig.ts").expect("Unable to create webConfig.ts");
    file.write_all(output.as_bytes())
        .expect("Unable to write to webConfig.ts");
    println!("webConfig.ts has been written.");
}
