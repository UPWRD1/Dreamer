extern crate serde;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    name: String,
    description: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RunConfig {
    run: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluConfig {
    project: ProjectConfig,
    r#do: RunConfig,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Read the .plu.yaml file
    let file = File::open("pipe.plu.yaml")?;
    let reader = BufReader::new(file);

    // Parse the YAML into PluConfig struct
    let config: PluConfig = serde_yaml::from_reader(reader)?;

    // Execute commands in the 'run' section
    for command in config.r#do.run {
        let mut parts = command.split_whitespace();
        let program = parts.next().ok_or("Missing command")?;
        let args: Vec<&str> = parts.collect();

        let status = Command::new(program)
            .args(args)
            .status()?;

        if status.success() {
            println!("Command '{}' executed successfully", command);
        } else {
            eprintln!("Error executing command '{}'", command);
        }
    }

    Ok(())
}
