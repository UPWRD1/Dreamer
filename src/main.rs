/// Main Entry Point
// Extern imports
extern crate serde;
extern crate serde_yaml;

// Local imports
pub mod helper;
use helper::{
    add, get_yaml_paths, help, init, invalid_args_notify, list, load, load_and_run,
    refs::{ADDCMD, HELPCMD, INITCMD, LISTCMD, LOADCMD, RUNCMD},
    resource::{argparse, print_file_list_main, quit, verbose_info_print},
    run, verbose_set_true,
};

// std imports
use std::env::{self};
use std::iter::*;
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
    let mut global_options: Vec<bool> = vec![];
    /*
    global options:
    0: verbose
    1:
     */
    verbose_set_true(&args, &mut global_options);
    if args.clone().len() == 1 {
        let n_args = synth_args(&args);
        let n_args_u = n_args.clone().expect("Asdf");
        let n_args_str: Vec<&str> = n_args_u.iter().map(|x| String::as_str(&x)).collect();
        let n_args_string = n_args_str
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();
        match load_and_run(n_args_string, ENV_COMMANDS, home_dir, &global_options) {
            Ok(()) => {
                let n_args_u = n_args.clone().expect("Asdf");
                let n_args_str: Vec<&str> = n_args_u.iter().map(|x| String::as_str(&x)).collect();
                let n_args_string = n_args_str
                    .iter()
                    .map(|&s| s.to_string())
                    .collect::<Vec<String>>();

                match run(&n_args_string, &global_options) {
                    Ok(()) => verbose_info_print("OK".to_string(), &global_options),
                    Err(..) => quit(),
                }
            }
            Err(..) => quit(),
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
                let _ = run(&args, &global_options);
            }
            _ if argparse(&args, 1, HELPCMD) => {
                help(args);
            }
            _ if argparse(&args, 1, LOADCMD) => {
                let _ = load(args, ENV_COMMANDS, home_dir, &global_options);
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

fn synth_args(args: &Vec<String>) -> Result<Vec<String>, ()> {
    match print_file_list_main() {
        Ok(index_c) => {
            let index_u = index_c.0.to_digit(10).unwrap() as usize;
            let mut n_args = args.clone();
            n_args.push("".to_string());
            n_args.insert(
                2,
                index_c.1[index_u - 1]
                    .clone()
                    .strip_suffix(".uni")
                    .unwrap()
                    .to_string(),
            );
            Ok(n_args)
        }

        Err(()) => {
            quit();
            Err(())
        }
    }
}

fn main() {
    cli();
}
