use colored::*;
use std::env;
use templates::Templates;
mod creation;
use creation::*;
mod operations;
mod templates;
use operations::*;
//HetzWGA

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let subcommand = if let Some(cmd) = args.first() {
        cmd
    } else {
        Templates::help();
        return Ok(());
    };

    if subcommand == "run" || subcommand == "r" {
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
    } else if subcommand == "new" || subcommand == "n" {
        let new_dir = if let Some(dir) = args.iter().nth(1) {
            dir
        } else {
            println!("{} a new project :^(", "Couldn't create".red());
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "`jargo new` was called but no directory was specified",
            ));
        };

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
                            println!(
                                "{} `{}`",
                                "Created new application".green().bold(),
                                new_dir.cyan()
                            );
                        }
                        Err(e) => println!("error: {}", e),
                    }
                }
                Err(e) => println!("error: {}", e),
            },
            Err(e) => println!("error: {}", e),
        };
    } else if subcommand == "create" || subcommand == "c" {
        let classname = if let Some(class) = args.iter().nth(1) {
            class
        } else {
            println!("{} a new class :^(", "Couldn't create".red());
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "`jargo create` was called but no directory was specified",
            ));
        };

        match CreationController::create_class(classname) {
            Ok(_) => println!("{} `{}`", "Created class:".green().bold(), classname.cyan()),
            Err(e) => println!("{}", e),
        }
    } else if subcommand == "jrun" || subcommand == "j" {
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
    } else if subcommand == "help" || subcommand == "h" {
        Templates::help();
    } else {
        println!("Unknown command")
    }
    Ok(())
}
