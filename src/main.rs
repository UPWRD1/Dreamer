/// Main code.
extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::{argparse, help, init, invalid_args_notify, load, run, list};

use helper::refs::{HELPCMD, INITCMD, LOADCMD, RUNCMD};

use crate::helper::add;
use crate::helper::refs::{LISTCMD, ADDCMD};

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
        let _ = load(args, ENV_COMMANDS, home_dir);
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
                let _ = list(args);
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
