/// Primary Parsing and Logic Functions.
extern crate colored;
use crate::helper::colored::Colorize;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
pub mod resource;
pub mod shell;
use crate::helper::resource::{
    check_arg_len, clear_term, extrahelp, input_fmt, matchcmd, printhelp, printusage,
    printusetemplate, quit, read_file, usage_and_quit, continue_prompt,
};

pub(crate) mod refs;
use crate::helper::refs::*;
pub mod exec;
use crate::helper::exec::*;
pub mod wizards;
use serde::{Deserialize, Serialize};
use wizards::*;
//use std::env;
use std::error::Error;
//use std::fs::metadata;
use std::env::{self};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::iter::*;
use std::path::Path;
use std::path::PathBuf;

use self::refs::AVAILABLE_CMDS;
use self::resource::{read_file_gpath, bad_file_error};
use self::shell::init_shell;

pub const SELF_VERSION: &str = "2023 (0.1.0)";

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    description: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tool {
    name: String,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepsConfig {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunConfig {
    run: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniConfig {
    project: ProjectConfig,
    r#do: RunConfig,
    deps: DepsConfig,
}

fn usage(cmd: &str) {
    printusage(matchcmd(cmd).unwrap().usage);
}
/*
fn usagenb(cmd: &str) {
    printusagenb(matchcmd(cmd).unwrap().usage);
}
*/
fn createfile(ufile_name: String) -> Result<std::string::String, std::string::String> {
    infoprint!("Creating unifile: {}", ufile_name);
    let mut ufile = File::create(ufile_name).expect("[!] Error encountered while creating file!");
    ufile
        .write_all(
            b"project: {
  name: \"\",
  description: \"\",
  version: \"0.0.0\",
}

do:
  run:
    - echo hello world

deps:
  tools:",
        )
        .expect("[!] Error while writing to file");

    Ok("File Created!".to_string())
}

pub fn run(argsv: Vec<String>, global_opts: &[bool]) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(RUNCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2, RUNCMD) {
        Ok(v_file) => run_exec(v_file.0, v_file.1, global_opts.to_vec()),
        Err(file) => {
            errprint!("Cannot find file '{}'", file.1);
            infoprint!(
                "Help: Try 'unify init {}' to create a new uni.yaml file.",
                file.1
            );
            Ok(())
        }
    };
    Ok(())
}

pub fn help(argsv: Vec<String>) {
    if argsv.len() == 2 {
        infoprint!("Unify is a project dependancy grabber\n\tVersion: {}\n",
            SELF_VERSION
        );
        printusetemplate();
        infoprint!("{}", "Commands:".bold());
        for x in AVAILABLE_CMDS {
            print!("\t - ");
            printhelp(x);
        }
        println!("");
        infoprint!("For more information on a command, run {}", "'unify help <command>'".black());
    } else {
        extrahelp(argsv[2].as_str());
    }
}

pub fn init(argsv: Vec<String>, global_opts: &[bool]) -> Result<std::string::String, std::string::String> {
    if argsv.len() == 3 {
        let ufile_name: String = format!("{}.uni.yaml", &argsv[2]).to_owned();
        let ufile_name_str: &str = &ufile_name[..];

        if Path::new(ufile_name_str).exists() {
            errprint!("File {} already Exists!", ufile_name);
            continue_prompt(global_opts);
            let _ = createfile(ufile_name);
            Ok("OK".to_string())
        } else {
            let _ = createfile(ufile_name);
            Ok("OK".to_string())
        }
    } else {
        usage_and_quit(INITCMD.name, "Invalid arguments!");
        Err("Invalid Arguments!".to_string())
    }
}

pub fn load(
    argsv: Vec<String>,
    env_cmds: Vec<String>,
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    match load_deps(
        argsv.to_owned(),
        &env_cmds.to_vec(),
        home_dir.clone(),
        global_opts,
    ) {
        Err(_) => {
            quit();
            Err("Error Loading".into())
        }
        Ok(result) => {
            init_shell(result.0.clone(), home_dir.clone(), result.1);
            Ok(())
        }
    }
}

pub fn load_and_run(
    argsv: Vec<String>,
    env_cmds: Vec<String>,
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    match load_deps(
        argsv.to_owned(),
        &env_cmds.to_vec(),
        home_dir.clone(),
        global_opts,
    ) {
        Err(_) => {
            quit();
            Err("Error Loading".into())
        }
        Ok(result) => match run(argsv, global_opts) {
            Err(_) => {
                quit();
                Err("Error Running".into())
            }
            Ok(..) => {
                init_shell(result.0, home_dir, result.1);
                Ok(())
            }
        },
    }
}
fn list_exec(v_file: File, filepath: String, way: usize) -> Result<(), Box<dyn Error>> {
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

            _ => {
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

pub fn list(argsv: Vec<String>, way: usize) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(LISTCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2, LISTCMD) {
        Ok(v_file) => {
            let result = list_exec(v_file.0, v_file.1, way);
            Ok(result)
        }
        Err(file) => {
            bad_file_error(&file.1);
            Err(())
        }
    };
    Err("Bad File".into())
}

pub fn add(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        //usage_and_quit(ADDCMD.name, "Missing Arguments!")
        match add_cmd_wizard() {
            Ok(vals) => {
                let _ = add_exec(&vals.0, &vals.1);
                Ok(())
            }

            Err(err) => {
                Err(err)
            }
        }
    } else {
        let dep_to_get = &argsv[2];

        let _ = match read_file_gpath(&argsv[3]) {
            Ok(v_file) => {
                let result = add_exec(&v_file.1, dep_to_get);
                Ok(result)
            }
            Err(file) => {
                errprint!("Cannot find file '{}'", file.1);
                infoprint!(
                    "Help: Try 'unify init {}' to create a new uni.yaml file.",
                    file.1
                );
                Err(())
            }
        };
        Err("Bad File".into())
    }
}

pub fn invalid_args_notify(args: Vec<String>) {
    errprint!(
        "{0}{1}{2}",
        "Invalid Command '".red().bold(),
        args[1].red().bold(),
        "'".red().bold()
    );
    infoprint!("Run 'unify help' to see available commands.");
}

pub fn argparse(argsv: &[String], pos: usize, cmd: Cmd) -> bool {
    // Parse arguments
    cmd.aliases.contains(&argsv[pos].as_str())
}

pub fn get_yaml_paths(dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|path| {
            if path
                .extension()
                .map_or(false, |ext| (ext == "yaml") || ext == "yml")
            {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}

pub fn verbose_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-v".to_string()) {
        global_opts.insert(0, true);
        global_opts.to_vec()
    } else {
        global_opts.to_vec()
    }
}


pub fn verbose_check(global_opts: &[bool]) -> bool {
    if global_opts.len() != 0 {
        global_opts[0]
    } else {
        false
    }
}

pub fn verbose_info_print(msg: String, global_opts: &[bool]) {
    if verbose_check(global_opts) {
        infoprint!("{msg}")
    }
}

pub fn force_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-f".to_string()) {
        global_opts.insert(1, true);
        global_opts.to_vec()
    } else {
        global_opts.to_vec()
    }
}

pub fn scan_flags(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    let unify_flags: Vec<&str> = vec!["-v", "-f"];
    for i in unify_flags {
        if argsv.contains(&i.to_owned().to_string()) {
            match i {
                "-v" => {
                    verbose_set_true(argsv, global_opts);
                },

                "-f" => {
                    force_set_true(argsv, global_opts);
                },

                &_ => {

                }
            }
        }
    }
    global_opts.to_vec()
}