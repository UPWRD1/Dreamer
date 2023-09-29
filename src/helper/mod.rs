extern crate colored;
extern crate serde;
extern crate serde_yaml;
//extern crate rand;

#[macro_use]
mod resource;
use crate::helper::resource::{throw_fatal, printusage, printusagenb, printusetemplate, printhelp, usage_and_quit, option_list, check_arg_len};

//use rand::prelude::*;
use serde::{Deserialize, Serialize};
//use std::array;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::iter::*;
use std::path::Path;
use std::process::Command;

pub const SELF_VERSION: &str = "2023 (0.1.0)";

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    description: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig {
    run: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniConfig {
    project: ProjectConfig,
    r#do: RunConfig,
}

pub struct Cmd<'a> {
    pub name: &'a str,
    pub desc: &'a str,
    pub usage: &'a str,
    pub aliases: [&'a str; 4],
}

pub const RUNCMD: Cmd = Cmd {
    name: "run",
    desc: "Executes a .uni.yaml file",
    usage: "run <filename>",
    aliases: ["run", "r", "--run", "-r"],
};

pub const HELPCMD: Cmd = Cmd {
    name: "help",
    desc: "This command",
    usage: "help",
    aliases: ["help", "h", "--help", "-h"],
};

pub const INITCMD: Cmd = Cmd {
    name: "init",
    desc: "Creates a new .uni.yaml file",
    usage: "init <filename>",
    aliases: ["init", "i", "--init", "-i"],
};

fn usage(cmd: &str) {
    match cmd {
        "help" => {
            printusage(HELPCMD.desc);
        }
        "run" => {
            printusage(RUNCMD.desc);
        }
        "init" => {
            printusage(INITCMD.desc);
        }
        &_ => {
            throw_fatal("Invalid command");
        }
    }
}

fn usagenb(cmd: &str) {
    match cmd {
        "help" => {
            printusagenb(HELPCMD.usage);
        }
        "run" => {
            printusagenb(RUNCMD.usage);
        }
        "init" => {
            printusagenb(INITCMD.usage);
        }
        &_ => errprint!(
            "{}",
            "FATAL ERROR [0004]: Invalid command.
        If you somehow see this, you probably need to reinstall unify, like now."
                .red()
                .bold()
        ),
    }
}

fn createfile(ufile_name: String) -> Result<std::string::String, std::string::String> {
    infoprint!("Creating unifile: {}", ufile_name);
    let mut ufile = File::create(ufile_name).expect("[!] Error encountered while creating file!");
    ufile
        .write_all(
            b"project: {
    name: '',
    description: '',
    version: '0.0.0',
}
do:
    run:
        - echo hello world
",
        )
        .expect("[!] Error while writing to file");

    Ok("File Created!".to_string())
}

pub fn run(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(RUNCMD.name, "Missing Filename!")
    }
    // Read the .plu.yaml file
    let index_to_open = 2;
    if index_to_open < argsv.len() {
        let filepath = argsv[index_to_open].to_string().to_owned() + ".uni.yml";
        let file = File::open(filepath.clone())?;
        let reader = BufReader::new(file);

        // Parse the YAML into PluConfig struct
        let config: UniConfig = serde_yaml::from_reader(reader)?;
        let mut okcount: i32 = 0;
        let mut cmdcount: i32 = 0;
        // Execute commands in the 'run' section

        infoprint!("Running '{}': \n", filepath);
        for command in config.r#do.run {
            cmdcount += 1;
            let mut parts = command.split_whitespace();
            let program = parts.next().ok_or("Missing command")?;
            let args: Vec<&str> = parts.collect();
            let status = Command::new(program).args(args).status()?;
            infoprint!("test");

            if status.success() {
                infoprint!("Command '{}' executed successfully", command);
                okcount += 1;
            } else {
                errprint!("Error executing command: '{}'", command);
            }
        }

        if cmdcount == okcount {
            println!();
            successprint!("All tasks completed successfully");
            println!();
        }
        Ok(())
    } else {
        errprint!("File '{}' not found!", argsv[2]);
        Err("Cannot find file".into())
    }
}

pub fn help() {
    print!("\t");
    println!(
        r"

          • ┏
    ┓┏ ┏┓ ┓ ╋━━┓┏
    ┗┻━┛┗━┗━┛  ┗┫
                ┛
    Version: {}                      
    ",
        SELF_VERSION
    );

    infoprint!("Unify is a project dependancy grabber");
    printusetemplate();
    println!();
    infoprint!("Commands:");
    printhelp(HELPCMD);
    printhelp(RUNCMD);
    printhelp(INITCMD);
}

pub fn init(argsv: Vec<String>) -> Result<std::string::String, std::string::String> {
    if argsv.len() == 3 {
        let ufile_name: String = format!("{}.uni.yaml", &argsv[2]).to_owned();
        let ufile_name_str: &str = &ufile_name[..];

        if Path::new(ufile_name_str).exists() {
            errprint!("File {} already Exists!", ufile_name);
            match questionprint!("Do you want to continue? (Y/N)").as_str() {
                "y" | "Y" => {
                    let _ = createfile(ufile_name);
                    Ok("OK".to_string())
                }
                &_ => {
                    errprint!("File creation aborted.");
                    usage_and_quit(INITCMD.name, "INVALID");
                    Ok("fail".to_string())
                }
            }
        } else {
            let _ = createfile(ufile_name);
            Ok("OK".to_string())
        }
    } else {
        usage_and_quit(INITCMD.name, "Invalid arguments!");
        Err("Invalid Arguments!".to_string())
    }
}

pub fn invalid_args_notify(args: Vec<String>) {
    errprint!(
        "{0}{1}{2}",
        "Invalid Command '".red().bold(),
        args[1].red().bold(),
        "'".red().bold()
    );
    eprintln!("Run 'unify help' to see available commands.");
}

pub fn argparse(argsv: Vec<String>, pos: usize, item_list: [&str; 4]) -> bool {
    // Parse arguments
    let x: String = argsv[pos].to_owned();
    let x_str: &str = &x[..];
    item_list.contains(&x_str)
}
