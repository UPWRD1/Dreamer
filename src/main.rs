//extern crate serde;
//extern crate serde_yaml;

use std::env::{self};
use std::iter::*;

mod helper;
use helper::argparse;
use helper::new;
use helper::run;
use helper::help;


pub fn cli() {
    // Main cli function
    let args: Vec<String> = env::args().collect(); // Argument collection
    // Parsing
    if argparse(args.clone(), 1, "new".to_string()) {
        new(args); // Create new plufile
    } else if argparse(args.clone(), 1, "run".to_string()) {
        let _ = run(args.clone()); // Run plufile
    } else if (argparse(args.clone(), 1,"help".to_string())) || 
              (argparse(args.clone(), 1, "--help".to_string())) ||
              (argparse(args.clone(), 1, "-h".to_string())) {
        let _ = help(); //help
    } else {
        println!("[!] Invalid Command '{}'. Run 'plumber help' to see available commands.", args[1]);
    }
}

fn main() {
    cli();
}
