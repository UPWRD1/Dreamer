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
use self::refs::COMMON_CMDS;
use self::shell::init_shell;
use std::env::{self};
use std::error::Error;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub const SELF_VERSION: &str = "2023 (0.1.0)";

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ProjectConfig {
    NAME: String,
    PACKAGE: String,
    DESCRIPTION: String,
    VERSION: String,
    IS_LOADED: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConfigToolInstallMethod {
    LINKZIP,
    GIT,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct ConfigTool {
    NAME: String,
    LINK: String,
    METHOD: ConfigToolInstallMethod,
}

pub struct TreeTool {
    tool: ConfigTool,
    dependancies: Vec<ConfigTool>
}

impl TreeTool {
    fn new(tool: ConfigTool) -> Self {
        TreeTool { tool, dependancies: vec![] }
    }
    
    fn add<T>(&mut self, tool: T) {
        where
            T: TreeTool,
            
        self.dependancies.push(tool)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct DepsConfig {
    TOOLS: Vec<ConfigTool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RunConfig {
    RUN: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ZzzConfig {
    PROJECT: ProjectConfig,
    ON: RunConfig,
    DEPENDANCIES: DepsConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum PromptItemKind {
    HOMEDIR,
    USRNAME,
    CURRDIR,
}

impl FromStr for PromptItemKind {
    type Err = ();

    fn from_str(input: &str) -> Result<PromptItemKind, Self::Err> {
        match input {
            "HOMEDIR" => Ok(PromptItemKind::HOMEDIR),
            "CURRDIR" => Ok(PromptItemKind::CURRDIR),
            "USRNAME" => Ok(PromptItemKind::USRNAME),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfigUser {
    name: String,
    prompt: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    user: UserConfigUser,
}

pub fn run(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(RUNCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&argsv, 2, RUNCMD) {
        Ok(v_file) => run_exec(v_file.0, v_file.1),
        Err(file) => {
            MISSINGFILEERROR.show_error(&file.1);
            Err("Missing File".into())
        }
    };
    Ok(())
}

pub fn help(argsv: Vec<String>, home_dir: Result<String, env::VarError>) {
    if (argsv.len() == 2) || (argsv.len() == 1) {
        infoprint!(
            "Dreamer is a project dependancy grabber\n\tVersion: {}\n",
            SELF_VERSION
        );
        printusetemplate();
        infoprint!("{}", "Common Commands:".bold());
        for x in COMMON_CMDS {
            print!("\t - ");
            printhelp(x);
        }
        println!();
        infoprint!(
            "For more information on a command, run {}",
            "'zzz help <command>'".black()
        );
        infoprint!(
            "To see all available commands, run {}",
            "'zzz help all'".black()
        );
    } else {
        extrahelp(argsv[2].as_str(), home_dir);
    }
}

pub fn new(argsv: Vec<String>) -> Result<std::string::String, std::string::String> {
    if argsv.len() == 3 {
        let zfile_name_f = &argsv[2];
        let ufile_name: String = format!("{}.zzz.yaml", zfile_name_f).to_owned();
        let ufile_name_str: &str = &ufile_name[..];

        if Path::new(ufile_name_str).exists() {
            errprint!("File {} already Exists!", ufile_name);
            continue_prompt();
            let _ = createfile(ufile_name, zfile_name_f);
            Ok("OK".to_string())
        } else {
            let _ = createfile(ufile_name, zfile_name_f);
            Ok("OK".to_string())
        }
    } else if argsv.len() == 2 {
        let zfile_name_f = "dream".to_string();
        let ufile_name: String = "dream.zzz.yaml".to_string();
        let ufile_name_str: &str = &ufile_name[..];

        if Path::new(ufile_name_str).exists() {
            errprint!("File {} already Exists!", ufile_name);
            continue_prompt();
            let _ = createfile(ufile_name, &zfile_name_f);
            Ok("OK".to_string())
        } else {
            let _ = createfile(ufile_name, &zfile_name_f);
            Ok("OK".to_string())
        }
    } else {
        usage_and_quit(NEWCMD.name, "Invalid arguments!");
        Err("Invalid Arguments!".to_string())
    }
}

pub fn start(
    argsv: Vec<String>,
    env_cmds: Vec<String>,
    home_dir: Result<String, env::VarError>,
) -> Result<(), Box<dyn Error>> {
    match load_deps(argsv.to_owned(), &env_cmds.to_vec(), home_dir.clone()) {
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

pub fn start_and_run(
    argsv: Vec<String>,
    env_cmds: Vec<String>,
    home_dir: Result<String, env::VarError>,
) -> Result<(), Box<dyn Error>> {
    match load_deps(argsv.to_owned(), &env_cmds.to_vec(), home_dir.clone()) {
        Err(_) => {
            quit(2);
            Err("Error Loading".into())
        }
        Ok(..) => match run(argsv.clone()) {
            Err(_) => {
                quit(2);
                Err("Error Running".into())
            }
            Ok(..) => Ok(()),
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
            MISSINGFILEERROR.show_error(&file.1);
            Err(())
        }
    };
    Err("Bad File".into())
}

pub fn add(argsv: Vec<String>) -> Result<(), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        match add_cmd_wizard() {
            Ok(vals) => {
                let _ = add_exec(&vals.0, &vals.1, vals.2);
                Ok(())
            }

            Err(err) => Err(err),
        }
    } else {
        Err("Bad File".into())
    }
}

pub fn extension(
    args: &Vec<String>,
    home_dir: Result<String, env::VarError>,
) -> Result<(), String> {
    if check_arg_len(args.clone(), 1) {
        usage_and_quit(EXTCMD.name, "No Extension!")
    }
    extension_exec(args, home_dir)
}

pub fn remove(args: Vec<String>) {
    if check_arg_len(args.clone(), 3) {
        let _ = remove_exec(&args[3], &args[2]);
    } else {
        match remove_cmd_wizard() {
            Ok(res) => {
                let _ = remove_exec(&res.0, &res.1);
            }
            Err(..) => {
                quit(4);
            }
        }
    }
}

pub fn forget(args: Vec<String>, home_dir: Result<String, env::VarError>) {
    if check_arg_len(args.clone(), 2) {
        usage_and_quit(FORGETCMD.name, "Missing Filename!")
    }

    let _ = match read_file(&args, 2, LISTCMD) {
        Ok(v_file) => {
            let result = forget_exec(home_dir, &v_file.1);
            Ok(result)
        }
        Err(file) => {
            MISSINGFILEERROR.show_error(&file.1);
            Err(())
        }
    };
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

fn argshelp_exec(s: Vec<char>, t: Vec<char>, way: usize) -> Result<String, String> {
    let (m, n) = (s.len(), t.len());
    match way {
        0 => {
            for i in 0..m {
                let mut j = 0;
                while j < n && s[i + j] == t[j] {
                    j += 1;
                    //break;
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
            for _i in 0..m {
                let mut j = 0;
                j += 1;
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
    Err("notfound".into())
}

pub fn argshelp(args: &[String], cmdtc: &Cmd) -> Result<String, String> {
    let t: Vec<char> = cmdtc.name.chars().collect();
    let s: Vec<char> = args[1].chars().collect();
    let (m, n) = (s.len(), t.len());
    if m < n {
        argshelp_exec(t, s, 1) // swap(t, s)
    } else {
        argshelp_exec(s, t, 0)
    }
}
