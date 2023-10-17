/// Command Execution
// Extern imports
use serde_yaml::Value;

// Local imports
use crate::{
    helper::{colored::Colorize, input_fmt, Tool, UniConfig},
    quit,
};

// std imports
use std::{error::Error, fs::File, io::BufReader};

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
    let f = File::open(filepath.clone())?;
    let reader: BufReader<File> = BufReader::new(f);
    // Parse the YAML into DepConfig struct
    let config: Result<UniConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            errprint!("File '{}' is not a valid config file", filepath);
            quit();
        }
        Ok(config) => {
            infoprint!("Getting dependancies from file: '{}'", filepath);
            let n_tool = Tool {
                name: depname.to_string(),
                link: link.clone(),
            };
            let path = filepath.as_str();
            let file = File::open(path).expect("File should exist");

            let mut value: Value = serde_yaml::from_reader(file).unwrap();

            value["deps"]["tools"] = t;

            let file = File::create(path).expect("File should exist");
            serde_yaml::to_writer(file, &value).unwrap();
        }
    }
    /*
    // Open a file with append option
    let mut data_file = std::fs::OpenOptions::new()
        .append(true)
        .open(filepath.clone())
        .expect("cannot open file");

    let cont = format!(
        "
    - name: \"{0}\"
      link: \"{1}\"",
        depname, link
    );
    // Write to a file
    data_file.write(cont.as_bytes()).expect("write failed");
     */

    successprint!("{0} added to {1}", depname, &filepath);

    Ok(())
}
