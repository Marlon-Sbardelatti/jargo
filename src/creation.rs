use crate::templates;
use crate::templates::Templates;
use std::env;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub struct CreationController;

impl CreationController {
    pub fn create_root_dir(new_dir: &String) -> Result<(), io::Error> {
        match fs::create_dir(new_dir) {
            Ok(_dir) => {
                // println!("Directory '{}' created successfully.", new_dir);

                let jargo_path = format!("{}/Jargo.toml", new_dir);

                match fs::File::create(jargo_path) {
                    Ok(_) => match Self::create_sub_dir(new_dir, &"src".to_string()) {
                        Ok(_) => match Self::create_sub_dir(new_dir, &"out".to_string()) {
                            Ok(_) => return Ok(()),
                            Err(e) => println!("error: {}", e),
                        },
                        Err(e) => println!("error: {}", e),
                    },
                    Err(e) => {
                        println!("Could not create Jargo.toml file, error: {}", e);
                    }
                }
            }
            Err(e) => println!("Error creating directory: {}", e),
        };

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not create the project",
        ))
    }

    pub fn create_sub_dir(new_dir: &String, sub_dir: &String) -> Result<(), io::Error> {
        match env::current_dir() {
            Ok(path) => {
                let src_path = path.join(new_dir).join(sub_dir);
                match fs::create_dir(src_path) {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => println!("error: {}", e),
                }
            }
            Err(e) => println!("{}", e),
        };

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not create the project",
        ))
    }

    pub fn create_sub_files(
        dir: &String,
        sub_dir: &String,
        file_name: Option<&String>,
    ) -> Result<String, io::Error> {
        if let Some(file) = file_name {
            let path = format!("{}/{}/{}", dir, sub_dir, file);
            match fs::File::create(path) {
                Ok(mut file) => {
                    file.write_all(Templates::generate_main().as_bytes())?;

                    file.flush()?;

                    // println!("{} file was created", file);

                    return Ok(dir.to_string());
                }
                Err(e) => println!("error: {}", e),
            }
        } else {
            let path = format!("{}/{}", dir, sub_dir);
            match fs::create_dir(path) {
                Ok(_) => {
                    println!("Out dir was created");
                    return Ok(dir.to_string());
                }
                Err(e) => println!("error this onee: {}", e),
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not create the project",
        ))
    }

    pub fn create_class(classname: &String) -> Result<(), io::Error> {
        match env::current_dir() {
            Ok(path) => {
                if path.join("src").exists() {
                    let classpath =
                        format!("{}/{}.java", path.join("src").to_string_lossy(), classname);
                    match fs::File::create_new(classpath) {
                        Ok(mut file) => {
                            file.write_all(Templates::generate_class(classname).as_bytes())?;

                            file.flush()?;

                            return Ok(());
                        }
                        Err(e) =>return Err(io::Error::new(io::ErrorKind::Other, format!("Could not create class, a class with this name already exists.\nerror: {e}"))),
                    }
                }
            }
            Err(e) => println!("error: {}", e),
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "Could not create class",
        ))
    }
}
