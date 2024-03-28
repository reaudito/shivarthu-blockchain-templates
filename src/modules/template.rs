use lazy_static::lazy_static;
use tera::Tera;

// pub const OUT_DIR: &str = "out_dir";

pub const OUT_DIR: &str = "out_dir";
// pub const PROJECT_DIR: &str = "project_dir";

pub const PROJECT_DIR: &str = "/home/amiya/Documents/workspace/shivarthu/working_directory/substrate-node-template/pallets";

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}
