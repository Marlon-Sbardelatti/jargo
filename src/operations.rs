use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub struct OperationController;

impl OperationController {
    pub fn find_toml() -> Result<PathBuf, io::Error> {
        match env::current_dir() {
            Ok(root_path) => match root_path.clone().file_name() {
                Some(dir) => {
                    match fs::read_dir(root_path.clone()) {
                        Ok(files) => {
                            println!("{:?}", root_path);
                            for f in files {
                                match f {
                                    Ok(file) => {
                                        let path = file.path();
                                        if let Some(file_name) = path.file_name() {
                                            if file_name == "Jargo.toml" {
                                                // println!("Filename: {:?}", file_name);
                                                return Ok(root_path);
                                            }
                                        }
                                    }
                                    Err(_) => todo!(),
                                }
                            }
                        }
                        Err(_) => todo!(),
                    }
                    // println!("The current directory name is {:?}", dir)
                }
                None => println!("Could not determine the directory name."),
            },
            Err(e) => {
                println!("Error getting current directory: {}", e);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find Jargo.toml file. Make sure you are in the right directory",
        ))
    }

    pub fn compile(root_path: PathBuf) -> Result<(), io::Error> {
        if root_path.join("src").exists() && root_path.join("out").exists() {
            let src_path = root_path.join("src");
            let out_path = root_path.join("out");

            let status = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "javac -d {} {}/*.java",
                    out_path.to_string_lossy(),
                    src_path.to_string_lossy()
                ))
                .status()
                .expect("Failed to execute javac");

            if status.success() {
                match Self::run(out_path) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => println!("error: {}", e),
                }
            } else {
                println!("Compilation failed");
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not compile the files",
                ));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not find src directory and/or out directory",
        ))
    }

    pub fn run(out_path: PathBuf) -> Result<(), io::Error> {
        let status = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "java -classpath {} Main",
                out_path.to_string_lossy()
            ))
            .status()
            .expect("Failed to execute java command");

        if status.success() {
            return Ok(());
        } else {
            println!("Compilation failed");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not run the files",
            ));
        }
    }

    pub fn find() -> Result<PathBuf, io::Error> {
        match env::current_dir() {
            Ok(root_path) => match Self::look_for_src(root_path) {
                Ok(src_path) => return Ok(src_path),
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unable to locate src directory\nerror: {}", e),
                    ))
                }
            },
            Err(e) => return Err(e),
        }
        // Err(io::Error::new(
        //     io::ErrorKind::Other,
        //     "Could not run the files",
        // ))
    }

    pub fn look_for_src(mut path: PathBuf) -> Result<PathBuf, io::Error> {
        if path.join("src").exists() {
            return Ok(path.to_path_buf());
        } else {
            while path.join("src").exists() == false {
                path = PathBuf::from(path.parent().unwrap());
            }
            return Ok(path.to_path_buf());
        }
    }
}
