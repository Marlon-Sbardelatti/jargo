use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use colored::Colorize;

pub struct OperationController;

impl OperationController {
    // NOTE: no changes so far (`find_toml` is never used)
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
        // define src + out dirs
        let src_path = root_path.join("src");
        let out_path = root_path.join("out");

        // if any of them doesn't exists, early returning
        if [&src_path, &out_path].iter().any(|cond| !cond.exists()) {
            println!("{}", "Compilation failed".red().bold());
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Couldn't find `src` and/or `out` directory",
            ));
        }

        // get java files
        let java_files: Vec<_> = fs::read_dir(src_path)?
            .filter_map(|f| f.ok())
            .filter(|f| f.path().extension().map_or(false, |ext| ext == "java"))
            .map(|f| f.path())
            .collect();

        // compile cmd
        let status = Command::new("javac")
            .arg("-d")
            .arg(&out_path)
            .args(java_files)
            .status()
            .expect("Failed to execute javac");

        // if not success, alert + early return
        if !status.success() {
            println!("Compilation failed");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not compile the files",
            ));
        }

        // else, print success + run
        println!("{}\n", "java project compiled successfully!".green().bold());
        match Self::run(out_path) {
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Couldn't run the files.\n{}: {}", "error".red(), e),
            )),
            _ => Ok(()),
        }
    }

    pub fn run(out_path: PathBuf) -> Result<(), io::Error> {
        // run cmd
        let status = Command::new("java")
            .arg("-classpath")
            .arg(&out_path)
            .arg("Main")
            .status()
            .expect("Failed to execute java command");

        // return cmd status info
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Couldn't run the {}/Main.class", out_path.to_string_lossy()),
            ))
        }
    }

    pub fn find() -> Result<PathBuf, io::Error> {
        // if current_dir is err, early returning
        let abs_path = match env::current_dir() {
            Ok(p) => p,
            e => return e,
        };

        // return src dir looking status
        match Self::look_for_src(abs_path) {
            Err(e) => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Unable to locate src directory\nerror: {}", e),
            )),
            x => x,
        }
    }

    pub fn look_for_src(path: PathBuf) -> Result<PathBuf, io::Error> {
        if path.join("src").exists() {
            return Ok(path.to_path_buf());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Couldn't find src dir at current path",
            ));
        }

        // BUG: This is an error prone approach. Considering that I'm not at a
        // jargo (sub)directory, it will build a new PathBuf recursively until
        // it raise an error on `path.parent().unwrap()`
        #[allow(unreachable_code)]
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
