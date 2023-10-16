/// Command Execution

// Local imports
use crate::{quit, helper::{UniConfig, input_fmt, colored::Colorize}};

// std imports
use std::{
    io::{BufReader, Write},
    fs::File,
    error::Error
};

pub fn list_exec(v_file: File, filepath: String, way: usize) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML
    let config: Result<UniConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            errprint!("Invalid Config file '{}'", filepath);
            quit();
            Err("Invalid Config".into())
        }

        Ok(config) => match way {
            1 => {
                infoprint!("'{}' requires the following dependancies:", filepath);
                let mut num = 1;
                for tool in config.deps.tools {
                    println!("  {0}: {1}", num, tool.name);
                    num += 1;
                }
                Ok(())
            }

            0 | _ => {
                infoprint!("Dependancies for {}:", filepath);
                let mut num = 1;
                for tool in config.deps.tools {
                    println!("  {0}: {1}", num, tool.name);
                    num += 1;
                }
                Ok(())
            }
        },
    }
}

pub fn add_exec(filepath: String, depname: &String) -> Result<(), Box<dyn Error>> {
    let link = questionprint!("Enter Link for '{}':", depname);
    println!("{depname}:{link}");
    println!("{filepath}");
    // Open a file with append option
    let mut data_file = std::fs::OpenOptions::new()
        .append(true)
        .open(filepath)
        .expect("cannot open file");

    let cont = format!(
        "
    - name: \"{0}\"
      link: \"{1}\"",
        depname, link
    );
    // Write to a file
    data_file.write(cont.as_bytes()).expect("write failed");

    println!("Appended content to a file");

    Ok(())
}