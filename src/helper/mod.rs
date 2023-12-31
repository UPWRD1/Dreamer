/// Primary Parsing and Logic Functions.
// Extern imports
extern crate colored;
extern crate serde;
extern crate serde_yaml;

use crate::helper::colored::Colorize;
use serde::{Deserialize, Serialize};

// Local imports
#[macro_use]
pub mod resource;
use crate::helper::resource::*;

pub mod shell;

pub(crate) mod refs;
use crate::helper::refs::*;

pub mod errors;
use crate::helper::errors::*;

pub mod exec;
use crate::helper::exec::*;

pub mod wizards;
use wizards::*;

// std imports
use std::env::{self};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use self::refs::AVAILABLE_CMDS;
use self::shell::init_shell;

pub const SELF_VERSION: &str = "2023 (0.1.0)";

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    description: String,
    version: String,
    isloaded: bool,
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
pub struct ZzzConfig {
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

pub fn run(argsv: Vec<String>, global_opts: &[bool]) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(RUNCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2, RUNCMD) {
        Ok(v_file) => run_exec(v_file.0, v_file.1, global_opts.to_vec()),
        Err(file) => {
            MISSINGFILEERROR.show_error(&file.1, global_opts);
            Err("Missing File".into())
        }
    };
    Ok(())
}

pub fn help(argsv: Vec<String>) {
    if (argsv.len() == 2) || (argsv.len() == 1) {
        infoprint!(
            "Dreamer is a project dependancy grabber\n\tVersion: {}\n",
            SELF_VERSION
        );
        printusetemplate();
        infoprint!("{}", "Commands:".bold());
        for x in AVAILABLE_CMDS {
            print!("\t - ");
            printhelp(x);
        }
        println!();
        infoprint!(
            "For more information on a command, run {}",
            "'zzz help <command>'".black()
        );
    } else {
        extrahelp(argsv[2].as_str());
    }
}

pub fn new(
    argsv: Vec<String>,
    global_opts: &[bool],
) -> Result<std::string::String, std::string::String> {
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
        usage_and_quit(NEWCMD.name, "Invalid arguments!");
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
            quit(3);
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
            quit(2);
            Err("Error Loading".into())
        }
        Ok(result) => match run(argsv, global_opts) {
            Err(_) => {
                quit(2);
                Err("Error Running".into())
            }
            Ok(..) => {
                init_shell(result.0, home_dir, result.1);
                Ok(())
            }
        },
    }
}

pub fn list(argsv: Vec<String>, way: usize, global_opts: &[bool]) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(LISTCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2, LISTCMD) {
        Ok(v_file) => {
            let result = list_exec(v_file.0, v_file.1, way, global_opts);
            Ok(result)
        }
        Err(file) => {
            INVALIDFILEERR.show_error(&file.1, global_opts);
            Err(())
        }
    };
    Err("Bad File".into())
}

pub fn add(argsv: Vec<String>, global_opts: &[bool]) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        match add_cmd_wizard() {
            Ok(vals) => {
                let _ = add_exec(&vals.0, &vals.1, global_opts);
                Ok(())
            }

            Err(err) => Err(err),
        }
    } else {
        let dep_to_get = &argsv[2];
        match argsv.len() {
            4 => {
                let _ = match read_file_gpath(&argsv[3]) {
                    Ok(v_file) => {
                        let result = add_exec(&v_file.1, dep_to_get, global_opts);
                        Ok(result)
                    }
                    Err(file) => {
                        errprint!("Cannot find file '{}'", file.1);
                        infoprint!(
                            "Help: Try 'zzz new {}' to create a new uni.yaml file.",
                            file.1
                        );
                        Err(())
                    }
                };
            }
            _ => {
                usage_and_quit(ADDCMD.name, "Invalid arguments!");
            }
        }

        Err("Bad File".into())
    }
}

pub fn extension(args: Vec<String>, home_dir: Result<String, env::VarError>, global_opts: &[bool]) {
    if check_arg_len(args.clone(), 2) {
        usage_and_quit(EXTCMD.name, "No Extension!")
    }
    extension_exec(args, home_dir, global_opts)
}

pub fn remove(args: Vec<String>, global_opts: &[bool]) {
    if check_arg_len(args.clone(), 3) {
        let _ = remove_exec(&args[3], &args[2], global_opts);
    } else {
        match remove_cmd_wizard() {
            Ok(res) => {
                let _ = remove_exec(&res.0, &res.1, global_opts);
            }
            Err(..) => {
                quit(4);
            }
        }
    }
}

pub fn invalid_args_notify(args: Vec<String>) {
    errprint!(
        "{0}{1}{2}",
        "Invalid Command '".red().bold(),
        args[1].red().bold(),
        "'".red().bold()
    );
    for i in AVAILABLE_CMDS {
        match argshelp(&args, i) {
            Ok(..) => {
                break;
            }
            Err(..) => {
                continue;
            }
        }
    }

    infoprint!("Run 'zzz help' to see available commands.");
}

pub fn checkargs(argsv: &[String], pos: usize, cmd: Cmd) -> bool {
    cmd.aliases.contains(&argsv[pos].as_str())
}
/*
fn argshelp_exec(s: Vec<char>, t: Vec<char>, way: usize) -> Result<String, String> {
    let (m, n) = (s.len(), t.len());
    //println!("{m}");
    //println!("{n}");
    for i in 0..m {
        let mut j = 0;
        println!("{}", i+j);
        while j < n && s[i + j] == t[j] {
            j += 1;
        }
        if j == n {
            if n == m {
                println!("{:?} = {:?}", s, t);
            } else {
                match way {
                    0 => {
                        tipprint!("Did you mean {}?", String::from_iter(t));
                        return Ok("found".into())
                },
                    _ => {
                        tipprint!("Did you mean {}?", String::from_iter(s));
                        return Ok("found".into())
                    },
                }

            }
            return Err("notfound".into())
        }
    }
    return Err("notfound".into())
}
*/

fn argshelp_exec(s: Vec<char>, t: Vec<char>, way: usize) -> Result<String, String> {
    let (m, n) = (s.len(), t.len());
    //println!("{m}");
    //println!("{n}");
    match way {
        0 => {
            for i in 0..m {
                let mut j = 0;
                //println!("{}", i + j);
                while j < n && s[i + j] == t[j] {
                    j += 1;
                    break;
                }
                if j == n {
                    if n == m {
                        println!("{:?} = {:?}", s, t);
                        break;
                    } else {
                        tipprint!("Did you mean {}?", String::from_iter(t));
                        return Ok("found".into());
                    }
                    //return Err("notfound".into());
                }
            }
        }
        _ => {
            for i in 0..m {
                let mut j = 0;
                //println!("{}", i + j);
                //println!("{:?}", s);
                while j < n && s[i + j] == t[j] {
                    j += 1;
                }
                if j == n {
                    if n == m {
                        println!("{:?} = {:?}", s, t);
                    } else {
                        tipprint!("Did you mean {}?", String::from_iter(s));
                        return Ok("found".into());
                    }
                    return Err("notfound".into());
                }
            }
        }
    }
    return Err("notfound".into());
}

pub fn argshelp(args: &Vec<String>, cmdtc: &Cmd) -> Result<String, String> {
    let t: Vec<char> = cmdtc.name.chars().collect();
    let s: Vec<char> = args[1].chars().collect();
    let (m, n) = (s.len(), t.len());
    if m < n {
        //println!("a");
        return argshelp_exec(t, s, 1); // swap(t, s)
    } else {
        //println!("b");
        return argshelp_exec(s, t, 0);
    }
}
