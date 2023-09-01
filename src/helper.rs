//extern crate serde;
//extern crate serde_yaml;

//use serde::{Deserialize, Serialize};
/*
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
*/

pub fn run() {
    println!("Runnning");
}

pub fn help() {
    print_sep(100);
    println!(r"                   
     _____ _           _           
    |  _  | |_ _ _____| |_ ___ ___ 
    |   __| | | |     | . | -_|  _|
    |__|  |_|___|_|_|_|___|___|_|                                
    ");
    println!("Plumber is a universal project manager. \n");
    print_sep(100);
    println!("");
    println!("Usage: plumber [COMMAND] [OPTIONS] \n");
    println!("Commands:");
    opt_print("help", "This command", "help");
    opt_print("new", "Create a new pipeline.", "new <pipelineName>");
    opt_print("add", "Brings up a dialog to add tasks to a pipeline", "add");
    opt_print("run", "Executes tasks found in a .plu.yaml file.", "run <optional: path/to/.plu.yaml>");
}

fn print_sep(num: usize) {
    let bar = "-".repeat(num);
    println!("{}", bar);
}

fn opt_print(name: &str, desc: &str, usage: &str) {
    // let barlen = 128;
    println!("\t {0} \t Desc: {1} \n \t \t Usage: plumber {2} \n", 
        name.to_string(), 
        desc.to_string(),
        usage.to_string()
        );
}