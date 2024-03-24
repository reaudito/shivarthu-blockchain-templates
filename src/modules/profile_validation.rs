use crate::modules::template::{OUT_DIR, TEMPLATES};
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::{Context, Result};

pub fn profile_validation() {
    let mut context = Context::new();
    context.insert("params_variable", &"profile_user_account");
    context.insert("params_variable_type", "AccountId");
    context.insert("params_type", "account");
    context.insert("schelling_game_name", "profile-validation");
    context.insert("trait_name", &"ProfileValidationApi");
    context.insert("runtime_pallet_name", &"ProfileValidation");
    context.insert("underscore_name", &"profile_validation");
    let save_directory = "profile_validation";
    let template_dir = "src/templates/schelling_game_templates";
    let template_folder = "schelling_game_templates";
 // Read the directory
 if let Ok(entries) = fs::read_dir(template_dir) {
    // Iterate over the entries
    for entry in entries {
        if let Ok(entry) = entry {
            // Check if it's a file (not a directory)
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                // Get the file name as a String
                if let Some(file_name) = entry.file_name().to_str() {
                    let template_name = format!("{}/{}", template_folder, file_name);
                    println!("{}", template_name);
                    match TEMPLATES.render(&template_name, &context) {
                        Ok(s) => {
                            let directory_path = format!("{}/{}", OUT_DIR, save_directory);

                            if let Err(err) = fs::create_dir_all(directory_path.clone()) {
                                eprintln!("Error creating directory: {}", err);
                                ::std::process::exit(1);
                            }
                            let file_path = Path::new(&directory_path).join(file_name);

                            // println!("{:?}", s);
                            if let Err(err) = fs::write(&file_path, s) {
                                eprintln!("Error writing to file: {}", err);
                                ::std::process::exit(1);
                            }

                            println!(
                                "Template rendered successfully. Output written to: {:?}",
                                file_path
                            );
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            let mut cause = e.source();
                            while let Some(e) = cause {
                                println!("Reason: {}", e);
                                cause = e.source();
                            }
                        }
                    };
                }
            }
        }
    }
} else {
    eprintln!("Error reading directory: {}", template_dir);
}
}
