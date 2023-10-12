/// Main code.
extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::{argparse, help, init, invalid_args_notify, load, run, list, get_yaml_paths, add};
use helper::refs::{HELPCMD, INITCMD, LOADCMD, RUNCMD, LISTCMD, ADDCMD};
use helper::resource::throw_fatal;

use crate::helper::resource::option_list;
/*
Error codes:
0000 OK
0001 File not found
0002 Could not read file
0003 Invalid Arguments
0004 Internal Error
*/

pub fn cli() {
    // Main cli function
    let args: Vec<String> = env::args().collect(); // Argument collection
    let home_dir: Result<String, env::VarError> = env::var("HOME");
    pub const ENV_COMMANDS: Vec<String> = vec![];
    if args.clone().len() == 1 {
        match env::current_dir() {
            Ok(dir) => {
                match get_yaml_paths(dir.into_os_string().into_string().unwrap().as_str()) {
                    Ok(paths) => {
                        let paths_f = paths.into_iter().map(|s| s.as_path().to_str().map(|s| s.to_string()).unwrap()).collect();
                        option_list("info", paths_f, "Choose a file:");
                    }
                    Err(..) => {
                        throw_fatal("Very bad 2")
                    }
                }
            }
            Err(e) => {
                throw_fatal(format!("Very Bad: {e}").as_str())
            }
        }
        /* 
        let mut n_args = args.clone();
        n_args.push("".to_string());
        n_args.insert(2, "unify".to_string());
        let _ = load(n_args.clone(), ENV_COMMANDS, home_dir);
        let _ = run(n_args.clone());*/

    } else {
        match args[1] {
            _ if argparse(&args, 1, INITCMD) => {
                let _ = init(args);
            }
            _ if argparse(&args, 1, RUNCMD) => {
                let _ = run(args);
            }
            _ if argparse(&args, 1, HELPCMD) => {
                help(args);
            }
            _ if argparse(&args, 1, LOADCMD) => {
                load(args, ENV_COMMANDS, home_dir);
            }
            _ if argparse(&args, 1, LISTCMD) => {
                let _ = list(args, 0);
            }
            _ if argparse(&args, 1, ADDCMD) => {
                let _ = add(args);
            }
            _ => invalid_args_notify(args), // Create new plufile
        }
    }

    /*
    if args.clone().len() == 1 {
        //help();
        init_shell()
    } else if argparse(args.clone(), 1, INITCMD.aliases) {
        let _ = init(args); // Create new plufile
    } else if argparse(args.clone(), 1, RUNCMD.aliases) {
        let _ = run(args.clone()); // Run plufile
    } else if argparse(args.clone(), 1, HELPCMD.aliases) {
        help(); //help
    } else {
        invalid_args_notify(args);
    }
    */
}

fn main() {
    cli();
}
