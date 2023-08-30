use std::env;
use std::fs::File;
use std::io::*;
//use std::io::prelude::*;

pub fn run<String: std::fmt::Debug>(argslist: Vec<String>) {
    
}

pub fn new<String: std::fmt::Debug>(argslist: Vec<String>) {
    println!("New pipe: {:?}", argslist[2]);
    let _pipe_name = &argslist[2];
    let mut file = File::create("myPipe.plu.yaml").expect("Error encountered while creating file!");
    file.write_all(b"do: { \n \t echo hello world!\n }").expect("Error while writing to file");
}

pub fn argparse(argsv: Vec<String>, pos: usize, item: String) -> bool {
    if argsv.len() > 1 && argsv[pos] == item {
        return true;
    } else {
        return false;
    }
}

pub fn cli() {
    let args: Vec<String> = env::args().collect();
    if argparse(args.clone(), 1, "new".to_string()) {
        new(args);
    } else if argparse(args.clone(), 1, "run".to_string()) {
        run(args);
    }
/*
    if args.len() > 1 && args[1] == "new" {
        new(args)
    } else {
        println!("Invalid");
    }
    */
}

fn main() {
    cli()
}
