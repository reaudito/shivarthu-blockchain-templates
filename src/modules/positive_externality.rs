use crate::modules::template::{OUT_DIR, TEMPLATES};
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::{Context, Result};

pub fn positive_externality() {
    let mut context = Context::new();
    let get_key = r#"let pe_block_number =
    <ValidationPositiveExternalityBlock<T>>::get(user_to_calculate.clone());

    let key = SumTreeName::PositiveExternality {
        user_address: user_to_calculate,
        block_number: pe_block_number.clone(),
    };"#;
    context.insert("params_variable", &"user_to_calculate");
    context.insert("params_variable_pallet_function_type", "T::AccountId");
    context.insert("params_variable_type", "AccountId");
    context.insert("params_type", "account");
    context.insert("param_type_value", &"none");
    context.insert("schelling_game_name", "positive-externality");
    context.insert("runtime_pallet_name", &"PositiveExternality");
    context.insert("underscore_name", &"positive_externality");
    context.insert("rpc_url", &"positiveexternality");
    context.insert("get_key", &get_key);

    let save_directory = "positive_externality";
    
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
