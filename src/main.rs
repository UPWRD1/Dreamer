extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::{argparse, help, invalid_args_notify, init, run, INITCMD, RUNCMD, HELPCMD };

/*
Error codes:
0 OK
1 File not found
2 Could not read file
3 Null Filename

*/

pub fn cli() {
    // Main cli function
    let args: Vec<String> = env::args().collect(); // Argument collection
                                                   // Parsi

    if args.clone().len() == 1 {
        let _ = help();
    } else if argparse(args.clone(), 1, INITCMD.aliases) {
        let _ = init(args); // Create new plufile
    } else if argparse(args.clone(), 1, RUNCMD.aliases) {
        let _ = run(args.clone()); // Run plufile
    } else if argparse(args.clone(), 1, HELPCMD.aliases) {
        let _ = help(); //help
    } else {
        invalid_args_notify(args);
    }
}

fn main() {
    cli();
}
