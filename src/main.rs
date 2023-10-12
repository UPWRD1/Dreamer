/// Main code.
extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::refs::{ADDCMD, HELPCMD, INITCMD, LISTCMD, LOADCMD, RUNCMD};
use helper::resource::throw_fatal;
use helper::{add, argparse, get_yaml_paths, help, init, invalid_args_notify, list, load, run};

use crate::helper::resource::{option_list, quit};
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
            Ok(dir) => match get_yaml_paths(dir.into_os_string().into_string().unwrap().as_str()) {
                Ok(paths) => {
                    let paths_f: Vec<String> = paths
                        .into_iter()
                        .map(|s| {
                            s.file_stem()
                                .unwrap()
                                .to_str()
                                .map(|s| s.to_string())
                                .unwrap()
                        })
                        .collect();
                    let index = option_list("info", paths_f.clone(), "Choose a file (0 to quit):");
                    let index_s = index.parse::<usize>().unwrap();
                    if index_s < paths_f.len() {
                        if index_s == 0 {
                            quit();
                        } else {
                            let mut n_args = args.clone();
                            n_args.push("".to_string());
                            n_args.insert(
                                2,
                                paths_f[index_s - 1]
                                    .clone()
                                    .strip_suffix(".uni")
                                    .unwrap()
                                    .to_string(),
                            );
                            let _ = load(n_args.clone(), ENV_COMMANDS, home_dir);
                            let _ = run(n_args.clone());
                        }
                    } else {
                        quit()
                    }
                }
                Err(..) => throw_fatal("Very bad 2"),
            },
            Err(e) => throw_fatal(format!("Very Bad: {e}").as_str()),
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
