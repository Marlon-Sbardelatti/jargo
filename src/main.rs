use std::env;
use std::fmt::format;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
mod creation;
use creation::*;
mod operations;
mod templates;
use operations::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];

    if flag == "run" {
        match OperationController::find_toml() {
            Ok(root_path) => match OperationController::compile(root_path) {
                Ok(_) => {},
                Err(e) => println!("error: {}", e),
            },
            Err(e) => println!("error: {}", e),
        }
    } else if flag == "new" {
        let new_dir = &args[2];

        match CreationController::create_root_dir(new_dir) {
            Ok(_) => match env::current_dir() {
                Ok(path) => {
                    let path = format!("{}", path.join(new_dir).to_string_lossy());
                    match CreationController::create_sub_files(
                        &path,
                        &"src".to_string(),
                        Some(&"Main.java".to_string()),
                    ) {
                        Ok(_path) => {
                            println!("Created new application `{}`", new_dir);
                        }
                        Err(e) => println!("error: {}", e),
                    }
                }
                Err(e) => println!("error: {}", e),
            },
            Err(e) => println!("error: {}", e),
        };
    } else if flag == "create" {
        let classname = &args[2];
        match CreationController::create_class(classname) {
            Ok(_) => println!("Class {} created!", classname),
            Err(e) => println!("{}", e),
        } 
    }

    Ok(())
}

