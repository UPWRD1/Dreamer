/// Primary Parsing and Logic Functions.
extern crate colored;
use crate::helper::colored::Colorize;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
pub mod resource;
pub mod shell;
pub mod wizards;
use crate::helper::resource::{
    calculate_hash, check_arg_len, clear_term, hash_string, input_fmt, printhelp, printusage,
    printusagenb, printusetemplate, quit, usage_and_quit,
};

pub(crate) mod refs;
use crate::helper::refs::*;

use serde::{Deserialize, Serialize};
//use std::env;
use std::error::Error;
//use std::fs::metadata;
use std::env::{self};
use std::fs::{self, File};
use std::io::BufReader;
use std::io::Write;
use std::iter::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use self::refs::AVAILABLE_CMDS;
use self::resource::{extrahelp, matchcmd, read_file};
use self::shell::init_shell;
use self::wizards::init_cmd_wizard;

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

pub fn continue_prompt() {
    match questionprint!("Do you want to continue? (Y/N)").as_str() {
        "y" | "Y" => {}
        &_ => {
            quit();
        }
    }
}

fn usage(cmd: &str) {
    printusage(matchcmd(cmd).unwrap().usage);
}

fn usagenb(cmd: &str) {
    printusagenb(matchcmd(cmd).unwrap().usage);
}

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

fn run_exec(v_file: File, filepath: String, global_opts: Vec<bool>) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML into PluConfig struct
    let config: Result<UniConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            errprint!("Invalid Config file '{}'", filepath);
            quit();
            Err("Invalid Config".into())
        }

        Ok(config) => {
            let mut okcount: i32 = 0;
            let mut cmdcount: i32 = 0;
            // Execute commands in the 'run' section
            infoprint!("Running '{}': \n", filepath);
            for command in config.r#do.run {
                cmdcount += 1;
                let mut parts = command.split_whitespace();
                let program = parts.next().ok_or("Missing command")?;
                let args: Vec<&str> = parts.collect();
                if cfg!(target_os = "windows") {
                    let status = Command::new(program).args(args).status()?;
                    if status.success() {
                        if verbose_check(&global_opts) {
                            infoprint!("Command '{}' executed successfully", command);
                        }
                        okcount += 1;
                    } else {
                        errprint!("Error executing command: '{}'", command);
                    }
                } else {
                    let status = Command::new(program).args(args).status()?;
                    if status.success() {
                        if verbose_check(&global_opts) {
                            infoprint!("Command '{}' executed successfully", command);
                        }
                        okcount += 1;
                    } else {
                        errprint!("Error executing command: '{}'", command);
                    }
                }
            }

            if cmdcount == okcount {
                println!();
                successprint!("All tasks completed successfully");
                println!();
            }
            Ok(())
        }
    }
}

pub fn run(argsv: &Vec<String>, global_opts: &[bool]) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(RUNCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2) {
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
        print!("\t");
        println!(
            r"

          • ┏      Unify is a project dependancy grabber
    ┓┏ ┏┓ ┓ ╋━━┓┏
    ┗┻━┛┗━┗━┛  ┗┫  Version: {}
                ┛",
            SELF_VERSION
        );
        printusetemplate();
        infoprint!("Commands:");
        for x in AVAILABLE_CMDS {
            printhelp(x);
        }
    } else {
        extrahelp(argsv[2].as_str());
    }
}

pub fn init(argsv: Vec<String>) -> Result<std::string::String, std::string::String> {
    if argsv.len() == 3 {
        let ufile_name: String = format!("{}.uni.yaml", &argsv[2]).to_owned();
        let ufile_name_str: &str = &ufile_name[..];

        if Path::new(ufile_name_str).exists() {
            errprint!("File {} already Exists!", ufile_name);
            continue_prompt();
            let _ = createfile(ufile_name);
            Ok("OK".to_string())
        } else {
            let _ = createfile(ufile_name);
            Ok("OK".to_string())
        }
    } else {
        match init_cmd_wizard() {
            Ok(filename) => {
                let ufile_name: String = filename;
                let ufile_name_str: &str = &ufile_name[..];

                if Path::new(ufile_name_str).exists() {
                    errprint!("File {} already Exists!", ufile_name);
                    continue_prompt();
                    let _ = createfile(ufile_name);
                    Ok("OK".to_string())
                } else {
                    let _ = createfile(ufile_name);
                    Ok("OK".to_string())
                }
            }
            Err(..) => {
                usage_and_quit(INITCMD.name, "Invalid arguments!");
                Err("Invalid Arguments!".to_string())

            }
        }
    }
}

fn tool_install(
    tool: Tool,
    hashname: u64,
    env_cmds: &mut Vec<String>,
    home_dir: &mut Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    env_cmds.push(tool.name.clone());
    verbose_info_print(
        format!("Installing {0} from {1}", tool.name, tool.link),
        global_opts,
    );
    let link = tool.link;
    let link_str = format!("{}", link);
    if cfg!(windows) {
        let dir_loc = format!(
            "{0}\\.unify\\bins\\{1}\\",
            home_dir.as_mut().unwrap(),
            hashname
        );
        match fs::create_dir_all(&dir_loc) {
            Ok(..) => {
                let namef = format!("{0}{1}", dir_loc, tool.name);
                let args: Vec<&str> = vec!["/C", "curl", &link_str, "--output", &namef, "--silent"];
                //println!("{:?}", args);

                let status = Command::new("cmd").args(args).status()?;
                if status.success() {
                    let args2: Vec<&str> = vec!["/C", "chmod", "a+x", &namef];
                    let status2 = Command::new("cmd").args(args2).status()?;
                    if status2.success() {
                        verbose_info_print(format!("'{}' installed", tool.name), global_opts);
                        return Ok(());
                    } else {
                        errprint!("Error grabbing: '{}'", tool.name);
                        continue_prompt();
                        return Err("Error grabbing".into());
                    }
                    //infoprint!("Command '{}' executed successfully", command);
                } else {
                    errprint!("Error grabbing: '{}'", tool.name);
                    continue_prompt();
                    Err("Error grabbing".into())
                }
            }
            Err(..) => {
                errprint!("Error creating dir");
                Err("Error creating dir".into())
            }
        }
    } else {
        let dir_loc = format!("{0}/.unify/bins/{1}/", home_dir.as_mut().unwrap(), hashname);
        match fs::create_dir_all(&dir_loc) {
            Ok(..) => {
                let link_str_f = format!("{link_str}");
                let namef = format!("{0}{1}", dir_loc, tool.name);
                let args: Vec<&str> = vec!["-c", "/usr/bin/curl", &link_str_f, "--output", &namef];
                let status = Command::new("bash").args(args).status()?;

                if status.success() {
                    let args2: Vec<&str> = vec!["-c", "chmod", "a+x", &namef];
                    let status2 = Command::new("bash").args(args2).status()?;
                    if status2.success() {
                        verbose_info_print(format!("'{}' installed", tool.name), global_opts);
                        return Ok(());
                    } else {
                        errprint!("Error grabbing: '{}'", tool.name);
                        return Err("Error grabbing".into());
                    }
                } else {
                    errprint!("Error grabbing: '{}'", tool.name);
                    Err("Error grabbing".into())
                }
            }
            Err(..) => {
                errprint!("Error creating dir");
                Err("Error creating dir".into())
            }
        }
    }
}

fn load_exec(
    v_file: File,
    filepath: String,
    mut env_cmds: Vec<String>,
    mut home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML into DepConfig struct
    let config: Result<UniConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            errprint!("File '{}' is not a valid config file", filepath);
            quit();
            Err("Invalid Config".into())
        }
        Ok(config) => {
            infoprint!("Getting dependancies from file: '{}'", filepath);
            let hashname = calculate_hash(&config.project.name);
            println!("{}", hash_string(&config.project.name));
            for tool in config.deps.tools {
                let _ = tool_install(tool, hashname, &mut env_cmds, &mut home_dir, global_opts);
            }
            let result = (env_cmds, hashname);
            Ok(result)
        }
    }
}

fn load_deps(
    argsv: Vec<String>,
    env_cmds: &[String],
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(LOADCMD.name, "Missing Filename!");
        return Err("Bad File".into());
    } else {
        let _ = list(argsv.clone(), 1);
        infoprint!("This action will download the above, and run any tasks included.");
        continue_prompt();
        let _: Result<(Vec<String>, u64), ()> = match read_file(&argsv, 2) {
            Ok(v_file) => {
                let result =
                    load_exec(v_file.0, v_file.1, env_cmds.to_vec(), home_dir, global_opts);
                return result;
                //Ok(result)
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
    }
    Err("Bad File".into())
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

pub fn load_run(
    argsv: Vec<String>,
    env_cmds: Vec<String>,
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(), ()> {
    match load_deps(
        argsv.to_owned(),
        &env_cmds.to_vec(),
        home_dir.clone(),
        global_opts,
    ) {
        Err(_) => {
            quit();
            return Err(());
        }
        Ok(result) => match run(&argsv, global_opts) {
            Err(_) => {
                quit();
                return Err(());
            }
            Ok(..) => {
                init_shell(result.0, home_dir, result.1);
                return Ok(());
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

pub fn list(argsv: Vec<String>, way: usize) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(LISTCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2) {
        Ok(v_file) => {
            let result = list_exec(v_file.0, v_file.1, way);
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

fn add_exec(filepath: String, depname: &String) -> Result<(), Box<dyn Error>> {
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

pub fn add(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(ADDCMD.name, "Missing Arguments!")
    }
    let dep_to_get = &argsv[2];

    let _ = match read_file(&argsv, 3) {
        Ok(v_file) => {
            let result = add_exec(v_file.1, dep_to_get);
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
            if path.extension().map_or(false, |ext| ext == "yaml") {
                Some(path)
            } else {
                if path.extension().map_or(false, |ext| ext == "yml") {
                    Some(path)
                } else {
                    None
                }
            }
        })
        .collect::<Vec<_>>();
    Ok(paths)
}

pub fn verbose_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-v".to_string()) {
        global_opts.insert(0, true);
        return global_opts.to_vec();
    } else {
        return global_opts.to_vec();
    }
}

pub fn verbose_check(global_opts: &[bool]) -> bool {
    if global_opts.len() > 0 {
        return global_opts[0] == true;
    }
    return false;
}

pub fn verbose_info_print(msg: String, global_opts: &[bool]) {
    if verbose_check(global_opts) {
        infoprint!("{msg}")
    }
}
