/// Main Entry Point
// Extern imports
extern crate serde;
extern crate serde_yaml;

// Local imports
pub mod helper;
use helper::{
    add, extension, help, invalid_args_notify, list, start, new,
    refs::{ADDCMD, HELPCMD, LISTCMD, STARTCMD, NEWCMD, RUNCMD},
    resource::argparse,
    run,
};

// std imports
use std::env::{self};
use std::iter::*;

use crate::helper::{
    refs::{REMOVECMD, GRABCMD},
    remove,
    resource::scan_flags, grab,
};
/*
Error codes:
0000 OK
0001 Invalid Arguments
0002 File not found
0003 Bad File
0004 Fatal Internal Error
0005 Bad User Quit
0006 No files
*/

pub fn cli() {
    // Main cli function
    //env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect(); // Argument collection
    let home_dir: Result<String, env::VarError> = env::var("HOME");
    pub const ENV_COMMANDS: Vec<String> = vec![];

    let mut global_options: Vec<bool> = vec![false; 5];
    /*
    global options:
    0: verbose
    1: force
    2: clean
    3: dumb (no color)
     */
    scan_flags(&args, &mut global_options);
    //let env_args: &Vec<bool> = &global_options;
    if args.clone().len() == 1 {
        help(args, home_dir);
    } else {
        match args[1] {
            _ if argparse(&args, 1, NEWCMD) => {
                let _ = new(args, &global_options);
            }
            _ if argparse(&args, 1, RUNCMD) => {
                let _ = run(args, &global_options);
            }
            _ if argparse(&args, 1, HELPCMD) => {
                help(args, home_dir);
            }
            _ if argparse(&args, 1, STARTCMD) => {
                let _ = start(args, ENV_COMMANDS, home_dir, &global_options);
            }
            _ if argparse(&args, 1, LISTCMD) => {
                let _ = list(args, 0, &global_options);
            }
            _ if argparse(&args, 1, ADDCMD) => {
                let _ = add(args, &global_options);
            }
            _ if argparse(&args, 1, REMOVECMD) => {
                remove(args, &global_options);
            }
            _ if argparse(&args, 1, GRABCMD) => {
                let _ = grab(args, ENV_COMMANDS, home_dir, &global_options);
            }

            _ => match extension(&args, home_dir, &global_options) {
                Ok(..) => {},
                Err(..) => {
                    invalid_args_notify(args)
                }
            }, // Create new plufile
        }
    }
}

fn main() {
    cli();
}
