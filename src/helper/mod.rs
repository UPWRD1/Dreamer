extern crate colored;
extern crate serde;
extern crate serde_yaml;
//extern crate rand;

//use rand::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
//use std::array;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::Write;
use std::iter::*;
use std::process::Command;
use std::fmt::Arguments;
use std::io::{self, BufRead};

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

macro_rules! errprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0}  {1}","[!]".red().bold(), format_args!($($arg)*))
    }};
}

macro_rules! infoprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0}  {1}","[i]".blue().bold(), format_args!($($arg)*))
    }};
}
/*
macro_rules! warnprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0}  {1}", "[W]".yellow().bold(), format_args!($($arg)*))
    }};
}
*/
macro_rules! questionprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0} {1}", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

pub fn read_line_expect<B: BufRead>(src: &mut B) -> io::Result<String> {
    src.lines().next().map_or(
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Input BufRead reached EOF before".to_string(),
        )),
        |line| line,
    )
}

pub fn input_fmt<B: BufRead, W: Write>(
    src: &mut B,
    dst: &mut W,
    fmt: Arguments,
) -> io::Result<String> {
    dst.write_fmt(fmt)?;
    dst.flush()?;
    read_line_expect(src)
}

macro_rules! input {
    () => (read_line_expect(&mut ::std::io::stdin().lock()).unwrap());
    ($($arg:tt)*) => (input_fmt(&mut ::std::io::stdin().lock(), &mut ::std::io::stdout(), format_args!($($arg)*)).unwrap());
}


macro_rules! successprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0} {1}", "[✔]".green().bold(), format_args!($($arg)*))
    }};
}

fn printusage(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: {0}{1}", " ./unify ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: {0} {1}", " unify ".black(), msg.black());
    }
}

fn usage(cmd: &str) {
    match cmd {
        "help" => {
            printusage("help");
        }
        "run" => {
            printusage("run <filename>");
        }
        "new" => {
            printusage("new <filename>");
        }
        &_ => errprint!(
            "{}",
            "FATAL ERROR: Invalid command.
        If you somehow see this, you probably need to reinstall unify, like now."
                .red()
                .bold()
        ),
    }
}

fn printusagenb(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        println!("\t Usage: {0}{1}", " ./unify ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        println!("\t Usage: {0} {1}", " unify ".black(), msg.black());
    }
}

fn usagenb(cmd: &str) {
    match cmd {
        "help" => {
            printusagenb(&HELPCMD.usage);
        }
        "run" => {
            printusagenb(&RUNCMD.usage);
        }
        "init" => {
            printusagenb(&INITCMD.usage);
        }
        &_ => errprint!(
            "{}",
            "FATAL ERROR: Invalid command.
        If you somehow see this, you probably need to reinstall unify, like now."
                .red()
                .bold()
        ),
    }
}

fn check_arg_len(argsv: Vec<String>, lentocheck: usize) -> bool {
    if argsv.len() == lentocheck {
        return true;
    } else {
        return false;
    }
}

fn usage_and_quit(cmd: &str, msg: &str) {
    errprint!("{}", msg);
    usage(cmd);
    std::process::exit(0);
}

fn printhelp(cmd: Cmd) {
    infoprint!("{0} \t Info: {1}", cmd.name, cmd.desc);
    print!("\t");
    usagenb(&cmd.name);
}

fn printusetemplate() {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: unify [--version] [--help] <command> [arguments]");
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: unify [--version] [--help] <command> [arguments]");
    }
}

fn createfile(ufile_name: String) {
    infoprint!("Creating unifile: {}", ufile_name);
        let mut ufile =
            File::create(ufile_name).expect("[!] Error encountered while creating file!");
        ufile
            .write_all(
                b"
project: {
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

        return Ok("File Created!".to_string());
}

pub fn run(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit("run", "Missing Filename!")
    }
    // Read the .plu.yaml file
    let index_to_open = 2;
    if index_to_open < argsv.len() {
        let filepath = argsv[index_to_open].to_string().to_owned() + &".uni.yml".to_string();
        let file = File::open(filepath.clone())?;
        let reader = BufReader::new(file);

        // Parse the YAML into PluConfig struct
        let config: UniConfig = serde_yaml::from_reader(reader)?;

        let mut okcount: i32 = 0;
        let mut cmdcount: i32 = 0;
        // Execute commands in the 'run' section
        //let mut rng = thread_rng();

        infoprint!("Running '{}': \n", filepath);
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
    /*     if check_arg_len(argsv.clone(), 1) {
        usage_and_quit("help", "Invalid Arguments!")
    }
    if argsv.len() != 2 {
        usage("help");
    } */

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
            match input!("Do you want to continue? (Y/N)").as_str() {
                "y" | "Y" => { createfile(ufile_name); }
    &_ => {
        errprint!("INVALID");
        usage_and_quit(INITCMD.name, "INVALID");
    }
                }
            }
        }
        
    errprint!("Invalid arguments!");
    return Err("Invalid Arguments!".to_string());
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
    if item_list.contains(&x_str) {
        return true;
    } else {
        return false;
    }
}
