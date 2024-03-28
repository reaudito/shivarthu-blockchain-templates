use crate::modules::template::{OUT_DIR, PROJECT_DIR, TEMPLATES};
use std::error::Error;
use std::fs;
use std::path::Path;
use tera::{Context, Result};

pub fn project_tips() {
    let mut context = Context::new();
    let get_key = r#"let block_number = Self::get_block_number_of_schelling_game(project_id).unwrap();

            let key = SumTreeName::ProjectTips { project_id, block_number: block_number.clone() };"#;

    let schelling_game_name = "project-tips";
    context.insert("params_variable", &"project_id");
    context.insert("params_variable_pallet_function_type", "ProjectId");
    context.insert("params_variable_type", "ProjectId");
    context.insert("params_type", "number");
    context.insert("param_type_value", "u64");
    context.insert("schelling_game_name", &schelling_game_name);
    context.insert("runtime_pallet_name", &"ProjectTips");
    context.insert("underscore_name", &"project_tips");
    context.insert("rpc_url", &"projecttips");
    context.insert("get_key", &get_key);

    let save_directory = "project_tips";
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
                                if file_name == "rpc_lib.rs" {
                                    let directory_path_rpc = format!(
                                        "{}/{}/{}-rpc/src",
                                        PROJECT_DIR, schelling_game_name, schelling_game_name
                                    );

                                    if let Err(err) = fs::create_dir_all(directory_path_rpc.clone())
                                    {
                                        eprintln!("Error creating directory: {}", err);
                                        ::std::process::exit(1);
                                    }

                                    let file_path = Path::new(&directory_path_rpc).join("lib.rs");

                                    if let Err(err) = fs::write(&file_path, s.clone()) {
                                        eprintln!("Error writing to file: {}", err);
                                        ::std::process::exit(1);
                                    }
                                } else if file_name == "rpc_cargo.toml" {
                                    let directory_path_rpc = format!(
                                        "{}/{}/{}-rpc",
                                        PROJECT_DIR, schelling_game_name, schelling_game_name
                                    );

                                    if let Err(err) = fs::create_dir_all(directory_path_rpc.clone())
                                    {
                                        eprintln!("Error creating directory: {}", err);
                                        ::std::process::exit(1);
                                    }

                                    let file_path =
                                        Path::new(&directory_path_rpc).join("Cargo.toml");

                                    if let Err(err) = fs::write(&file_path, s.clone()) {
                                        eprintln!("Error writing to file: {}", err);
                                        ::std::process::exit(1);
                                    }
                                } else if file_name == "runtime_api_lib.rs" {
                                    let directory_path_rpc = format!(
                                        "{}/{}/{}-runtime-api/src",
                                        PROJECT_DIR, schelling_game_name, schelling_game_name
                                    );

                                    if let Err(err) = fs::create_dir_all(directory_path_rpc.clone())
                                    {
                                        eprintln!("Error creating directory: {}", err);
                                        ::std::process::exit(1);
                                    }

                                    let file_path = Path::new(&directory_path_rpc).join("lib.rs");

                                    if let Err(err) = fs::write(&file_path, s.clone()) {
                                        eprintln!("Error writing to file: {}", err);
                                        ::std::process::exit(1);
                                    }
                                } else if file_name == "runtime_api_cargo.toml" {
                                    let directory_path_rpc = format!(
                                        "{}/{}/{}-runtime-api",
                                        PROJECT_DIR, schelling_game_name, schelling_game_name
                                    );

                                    if let Err(err) = fs::create_dir_all(directory_path_rpc.clone())
                                    {
                                        eprintln!("Error creating directory: {}", err);
                                        ::std::process::exit(1);
                                    }

                                    let file_path =
                                        Path::new(&directory_path_rpc).join("Cargo.toml");

                                    if let Err(err) = fs::write(&file_path, s.clone()) {
                                        eprintln!("Error writing to file: {}", err);
                                        ::std::process::exit(1);
                                    }
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
