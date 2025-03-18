// src/choose.rs
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use rfd;

/// Stores the selected files.
pub struct FileChoices {
    pub banner_file: String,
    pub category_images: HashMap<String, [String; 3]>,
}

/// Prompts the user to choose a banner file and, for each product category,
/// asks for three image files using a system file dialog.
pub fn choose_files(categories: &[String]) -> FileChoices {
    let banner_file: String;
    #[cfg(target_os = "windows")]
    {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image Files", &["png", "jpg", "jpeg"])
            .set_title("Choose your banner poster file")
            .pick_file()
        {
            banner_file = path.display().to_string();
        } else {
            eprintln!("No banner file selected. Exiting.");
            std::process::exit(1);
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image Files", &["png", "jpg", "jpeg"])
            .set_title("Choose your banner poster file")
            .pick_file()
        {
            banner_file = path.display().to_string();
        } else {
            eprintln!("No banner file selected. Exiting.");
            std::process::exit(1);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image Files", &["png", "jpg", "jpeg"])
            .set_title("Choose your banner poster file")
            .pick_file()
        {
            banner_file = path.display().to_string();
        } else {
            eprintln!("No banner file selected. Exiting.");
            std::process::exit(1);
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        // Fallback for other operating systems (can be improved with other libraries or simple input)
        use dialoguer::Input;
        banner_file = Input::new()
            .with_prompt("Choose your banner poster file (enter file path)")
            .interact_text()
            .unwrap();
    }

    let mut category_images = HashMap::new();
    // For each category, ask for 3 image sources:
    for category in categories {
        println!("\nFor category '{}':", category);
        let mut images: [String; 3] = Default::default();
        for i in 0..3 {
            let prompt = format!("Choose image {} for '{}'", i + 1, category);
            let file_path: String;
            #[cfg(target_os = "windows")]
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image Files", &["png", "jpg", "jpeg"])
                    .set_title(&prompt)
                    .pick_file()
                {
                    file_path = path.display().to_string();
                } else {
                    eprintln!("No image selected for '{}'. Exiting.", category);
                    std::process::exit(1);
                }
            }
            #[cfg(target_os = "linux")]
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image Files", &["png", "jpg", "jpeg"])
                    .set_title(&prompt)
                    .pick_file()
                {
                    file_path = path.display().to_string();
                } else {
                    eprintln!("No image selected for '{}'. Exiting.", category);
                    std::process::exit(1);
                }
            }
            #[cfg(target_os = "macos")]
            {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image Files", &["png", "jpg", "jpeg"])
                    .set_title(&prompt)
                    .pick_file()
                {
                    file_path = path.display().to_string();
                } else {
                    eprintln!("No image selected for '{}'. Exiting.", category);
                    std::process::exit(1);
                }
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            {
                use dialoguer::Input;
                file_path = Input::new()
                    .with_prompt(&prompt)
                    .interact_text()
                    .unwrap();
            }
            images[i] = file_path;
        }
        category_images.insert(category.clone(), images);
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