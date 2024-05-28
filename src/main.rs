use std::env;
use colored::*;
use templates::Templates;
mod creation;
use creation::*;
mod operations;
mod templates;
use operations::*;
//HetzWGA

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];

    if flag == "run" || flag == "r"{
        // match OperationController::find_toml() {
        //     Ok(root_path) => match OperationController::compile(root_path) {
        //         Ok(_) => {}
        //         Err(e) => println!("error: {}", e),
        //     },
        //     Err(e) => println!("error: {}", e),
        // }
        match OperationController::find() {
            Ok(path) => match OperationController::compile(path) {
                Ok(_) => {}
                Err(e) => println!("error: {}", e),
            },
            Err(e) => println!("error: {}", e),
        }
    } else if flag == "new" || flag == "n" {
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
                            println!("{} `{}`","Created new application".green().bold(), new_dir.cyan());
                        }
                        Err(e) => println!("error: {}", e),
                    }
                }
                Err(e) => println!("error: {}", e),
            },
            Err(e) => println!("error: {}", e),
        };
    } else if flag == "create" || flag == "c" {
        let classname = &args[2];
        match CreationController::create_class(classname) {
            Ok(_) => println!("{} `{}`","Created class:".green().bold(), classname.cyan()),
            Err(e) => println!("{}", e),
        }
    } else if flag == "jrun" || flag == "j" {
        match OperationController::find() {
            Ok(path) => {
                let out_path = path.clone().join("out");
                match OperationController::run(out_path) {
                    Ok(_) => {}
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(e) => println!("error: {}", e),
        }
    } else if flag == "help" || flag == "h" {
        Templates::help();
    }
    Ok(())
}
