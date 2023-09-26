extern crate serde;
extern crate serde_yaml;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
//use std::io::*;
use std::io::Write;
use std::iter::*;
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

pub fn run(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    // Read the .plu.yaml file
    let index_to_open = 2;
    if index_to_open < argsv.len() {
        let filepath = argsv[index_to_open].to_string().to_owned() + &".plu.yaml".to_string();
        let file = File::open(filepath.clone())?;
        let reader = BufReader::new(file);

        // Parse the YAML into PluConfig struct
        let config: PluConfig = serde_yaml::from_reader(reader)?;

        let mut okcount: i32 = 0;
        let mut cmdcount: i32 = 0;
        // Execute commands in the 'run' section
        println!("[i] Executing '{}' \n", filepath);
        for command in config.r#do.run {
            cmdcount += 1;
            let mut parts = command.split_whitespace();
            let program = parts.next().ok_or("Missing command")?;
            let args: Vec<&str> = parts.collect();

            let status = Command::new(program).args(args).status()?;

            if status.success() {
                //println!("Command '{}' executed successfully", command);
                okcount += 1;
            } else {
                eprintln!("[!] Error executing command '{}'", command);
            }
        }

        if cmdcount == okcount {
            println!(
                "\n 
        [i] All tasks completed successfully"
            );
        }
        Ok(())
    } else {
        eprintln!("[!] File '{}' not found!", argsv[2]);
        Err("Cannot".into())
    }
}

pub fn help() {
    println!(
        r"                   

      • ┏    
┓┏ ┏┓ ┓ ╋━━┓┏
┗┻━┛┗━┗━┛  ┗┫
            ┛
                            
    "
    );
    println!("Unify is a universal project manager.");
    println!("Options:");
}

pub fn new(argsv: Vec<String>) -> Result<std::string::String, std::string::String> {
    if argsv.len() == 3 {
        let ufile_name: String = format!("{}.uni.yaml", &argsv[2]);
    println!("[i] New pipe: {}", ufile_name);
    let mut plufile =
        File::create(ufile_name).expect("[!] Error encountered while creating file!");
    plufile
        .write_all(b"do: { \n \t echo hello world!\n }")
        .expect("[!] Error while writing to file");

        return Ok("File Created!".to_string())
    }
    eprintln!("[!] Invalid arguments!");
    return Err("Invalid Arguments!".to_string())
}

pub fn argparse(argsv: Vec<String>, pos: usize, item: String) -> bool {
    // Parse arguments
    if argsv.len() > 1 && argsv[pos] == item {
        return true;
    } else {
        return false;
    }
}
