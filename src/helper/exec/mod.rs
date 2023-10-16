/// Command Execution
// Local imports
use crate::{
    helper::{colored::Colorize, input_fmt, resource::read_file_gpath, Tool, UniConfig},
    quit,
};

// std imports
use std::{
    error::Error,
    fs::File,
    io::BufReader,
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
    let link = questionprint!("Enter link for '{}':", depname);
    let _ = match read_file_gpath(&filepath) {
        Ok(v_file) => {
            let config: Result<UniConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            let mut conf_f = config.unwrap();

            let n_tool: Tool = Tool {
                name: depname.to_string(),
                link,
            };
            let mut tool_to_add: Vec<Tool> = vec![n_tool];
            //let to_w = conf_f.deps.tools.append(&mut tool_to_add);
            conf_f.deps.tools.append(&mut tool_to_add);
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(v_file.1)
                .expect("Couldn't open file");
            serde_yaml::to_writer(f, &conf_f).unwrap();
        }
        Err(file) => {
            errprint!("Cannot find file '{}'", file.1);
            infoprint!(
                "Help: Try 'unify init {}' to create a new uni.yaml file.",
                file.1
            );
        }
    };

    successprint!("'{0}' added to {1}", depname, &filepath);

    Ok(())
}
