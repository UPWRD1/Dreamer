/// Main Entry Point
// Extern imports
extern crate serde;
extern crate serde_yaml;

// Local imports
pub mod helper;
use helper::{
    add, extension, help, invalid_args_notify, list, load, new,
    refs::{ADDCMD, HELPCMD, LISTCMD, LOADCMD, NEWCMD, RUNCMD},
    resource::argparse,
    run,
};

// std imports
use std::env::{self};
use std::iter::*;

use crate::helper::{
    refs::{EXTCMD, REMOVECMD},
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

    let mut global_options: Vec<bool> = vec![false; 5];
    /*
    global options:
    0: verbose
    1: force
    2: dumb (no color)
     */
    scan_flags(&args, &mut global_options);
    if args.clone().len() == 1 {
        help(args);
    } else {
        match args[1] {
            _ if argparse(&args, 1, NEWCMD) => {
                let _ = new(args, &global_options);
            }
            _ if argparse(&args, 1, RUNCMD) => {
                let _ = run(args, &global_options);
            }
            _ if argparse(&args, 1, HELPCMD) => {
                help(args);
            }
            _ if argparse(&args, 1, LOADCMD) => {
                let _ = load(args, ENV_COMMANDS, home_dir, &global_options);
            }
            _ if argparse(&args, 1, LISTCMD) => {
                let _ = list(args, 0, &global_options);
            }
            _ if argparse(&args, 1, ADDCMD) => {
                let _ = add(args, &global_options);
            }
            _ if argparse(&args, 1, EXTCMD) => {
                extension(args, home_dir, &global_options);
            }

            _ if argparse(&args, 1, REMOVECMD) => {
                remove(args, &global_options);
            }

            _ => invalid_args_notify(args), // Create new plufile
        }
    }
}

fn main() {
    cli();
}
