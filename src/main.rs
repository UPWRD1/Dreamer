extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::{argparse, help, init, invalid_args_notify, run, HELPCMD, INITCMD, RUNCMD};

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

    //println!("{}", args.len()); // Parsi

    if args.clone().len() == 1 {
        help();
    } else if argparse(args.clone(), 1, INITCMD.aliases) {
        let _ = init(args); // Create new plufile
    } else if argparse(args.clone(), 1, RUNCMD.aliases) {
        let _ = run(args.clone()); // Run plufile
    } else if argparse(args.clone(), 1, HELPCMD.aliases) {
        help(); //help
    } else {
        invalid_args_notify(args);
    }
}

fn main() {
    cli();
}
