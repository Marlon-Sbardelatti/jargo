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
mod templates;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];

    if flag == "run" {
        match CreationController::find_toml() {
            Ok(_) => println!("success"),
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
    }
    // if flag == "-g" || flag == "--get" {
    //     let get_name = &args[2];
    //     let file = File::open("/home/hetzwga/.config/tstyle/themes.txt")?;
    //     let reader = BufReader::new(file);

    //     let mut vec: Vec<String> = Vec::new();
    //     for line in reader.lines() {
    //         let line = line?;
    //         vec.push(line);
    //     }
    //     for string in vec {
    //         // Split the string by the semicolon
    //         let parts: Vec<&str> = string.split(';').collect();

    //         // Ensure that the string contains both name and value
    //         if parts.len() == 2 {
    //             // Extract the name and value
    //             let name = parts[0];
    //             let value = parts[1];

    //             if name == get_name {
    //                 change_hex(value.to_string())?;
    //                 break;
    //             }
    //         } else {
    //             // Handle unexpected format
    //             println!("Invalid format for string: {}", string);
    //         }
    //     }
    // } else if flag == "-l" || flag == "--list" {
    //     let file = File::open("/home/hetzwga/.config/tstyle/themes.txt")?;
    //     let reader = BufReader::new(file);

    //     let mut vec: Vec<String> = Vec::new();
    //     for line in reader.lines() {
    //         let line = line?;
    //         vec.push(line);
    //     }
    //     for string in vec {
    //         let parts: Vec<&str> = string.split(';').collect();
    //         if parts.len() == 2 {
    //             // Extract the name and value
    //             let name = parts[0];
    //             let value = parts[1];
    //             println!("Colorscheme: {name} -- HEX {value}");
    //         } else {
    //             // Handle unexpected format
    //             println!("Invalid format for string: {}", string);
    //         }
    //     }
    // } else if flag == "-c" || flag == "--create" {
    //     let theme_name = &args[2];
    //     let hex = &args[3];
    //     let home_dir = dirs::home_dir();
    //     if let Some(home_dir) = home_dir {
    //         let mut dir_path = home_dir.to_string_lossy().into_owned();
    //         dir_path.push_str("/.config/tstyle");
    //         create_dir_all(&dir_path)?;

    //         let mut file_path = PathBuf::from(dir_path);
    //         file_path.push("themes.txt");

    //         let mut file = OpenOptions::new()
    //             .create(true)
    //             .append(true)
    //             .open(&file_path)?;

    //         writeln!(file, "{theme_name};{hex}")?;

    //         let file = File::open("/home/hetzwga/.config/tstyle/themes.txt")?;
    //         let reader = BufReader::new(file);

    //         let mut vec: Vec<String> = Vec::new();
    //         for line in reader.lines() {
    //             let line = line?;
    //             vec.push(line);
    //         }
    //     }
    // } else if flag == "-h" || flag == "--help" {
    //     println!("-g, --get                    get a colorscheme by its name");
    //     println!("USAGE:                     tstyle -g colorscheme_name\n");
    //     println!("-c, --create                 create a new colorscheme");
    //     println!("USAGE: tstyle -c           colorscheme_name 'colorscheme_hex'");
    //     println!("make sure 'colorscheme_hex' is on quotes\n");
    //     println!("-l, --list                    list all colorschemes available\n");
    //     println!("-h, --help                    help command to see all functions");
    // } else {
    //     println!("Wrong set of arguments. Type -h or --help for help")
    // }
    Ok(())
}

fn change_hex(hex: String) -> std::io::Result<()> {
    let home_dir = dirs::home_dir();
    if let Some(home_dir) = home_dir {
        let mut dir_path = home_dir.to_string_lossy().into_owned();
        dir_path.push_str("/.tmux/plugins/tmux");

        let mut file_path = PathBuf::from(dir_path);
        file_path.push("catppuccin-mocha.tmuxtheme");
        let file = File::open(file_path)?;

        let reader = BufReader::new(file);

        let mut vec: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            vec.push(line);
        }

        vec[4] = format!("thm_bg='{}'", hex);

        let mut new = File::create("/home/hetzwga/.tmux/plugins/tmux/catppuccin-mocha.tmuxtheme")?;

        for line in &vec {
            writeln!(&mut new, "{}", line)?;
        }

        Command::new("tmux")
            .arg("source-file")
            .arg("/home/hetzwga/.tmux.conf")
            .spawn()
            .expect("source command failed to start");

        return Ok(());
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Home directory not found",
        ));
    }
}
