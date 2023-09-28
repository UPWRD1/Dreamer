extern crate serde;
extern crate serde_yaml;
//extern crate rand;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::{argparse, help, invalid_args_notify, init, run};

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

    if args.clone().len() == 0 {
        let _ = help(args);
    } else if argparse(args.clone(), 1, "init") {
        let _ = init(args); // Create new plufile
    } else if (argparse(args.clone(), 1, "run"))
        || (argparse(args.clone(), 1, "-run"))
        || (argparse(args.clone(), 1, "-r"))
        || (argparse(args.clone(), 1, "r"))
    {
        let _ = run(args.clone()); // Run plufile
    } else if (argparse(args.clone(), 1, "help"))
        || (argparse(args.clone(), 1, "--help"))
        || (argparse(args.clone(), 1, "-h"))
        || (argparse(args.clone(), 1, "h"))
    {
        let _ = help(args.clone()); //help
    } else {
        invalid_args_notify(args);
    }
}

fn main() {
    cli();
}
