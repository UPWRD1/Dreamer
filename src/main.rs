/// Main Entry Point
// Extern imports
extern crate serde;
extern crate serde_yaml;

// Local imports
pub mod helper;
use helper::{
    add, extension, forget, help, invalid_args_notify, list, new,
    refs::{ADDCMD, HELPCMD, LISTCMD, NEWCMD, RUNCMD, STARTCMD},
    resource::argparse,
    run, start,
};

// std imports
use std::{env::{self}, vec};
use std::iter::*;

use crate::helper::{
    refs::{FORGETCMD, REMOVECMD},
    remove,
    resource::scan_flags,
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
    scan_flags(&args);
    if args.clone().len() == 1 {
        help(args, home_dir);
    } else {
        match args[1] {
            _ if argparse(&args, 1, NEWCMD) => {
                let _ = new(args);
            }
            _ if argparse(&args, 1, RUNCMD) => {
                let _ = run(args);
            }
            _ if argparse(&args, 1, HELPCMD) => {
                help(args, home_dir);
            }
            _ if argparse(&args, 1, STARTCMD) => {
                let _ = start(args, ENV_COMMANDS, home_dir);
            }
            _ if argparse(&args, 1, LISTCMD) => {
                let _ = list(args, 0);
            }
            _ if argparse(&args, 1, ADDCMD) => {
                let _ = add(args);
            }
            _ if argparse(&args, 1, REMOVECMD) => {
                remove(args);
            }
            _ if argparse(&args, 1, FORGETCMD) => {
                forget(args, home_dir);
            }
            _ => match extension(&args, home_dir) {
                Ok(..) => {}
                Err(..) => invalid_args_notify(args),
            },
        }
    }
}

fn main() {
    cli();
}
