use std::env;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct CreationController;

impl CreationController {
    pub fn find_toml() -> Result<(), io::Error> {
        match env::current_dir() {
            Ok(path) => match path.file_name() {
                Some(dir) => {
                    match fs::read_dir(path) {
                        Ok(files) => {
                            for f in files {
                                match f {
                                    Ok(file) => {
                                        let path = file.path();
                                        if let Some(file_name) = path.file_name() {
                                            if file_name == "Jargo.toml" {
                                                println!("Filename: {:?}", file_name);
                                                return Ok(());
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
            "Could not find Cargo.toml file. Make sure you are in the right directory",
        ))
    }


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
            Ok(_) => {
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

}
