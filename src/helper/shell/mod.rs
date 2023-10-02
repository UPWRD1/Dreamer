//use super::resource::throw_fatal;
use crate::helper::colored::Colorize;
use crate::helper::input_fmt;
use std::{process::Command, path::Path, env};

fn unish_loop() {
    loop {
        let curr_dir = env::current_dir();
        let input = shellprint!("(~{}) [unify] \n \t===> ", curr_dir.unwrap().to_string_lossy());
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        match command { 
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(root) {
                    errprint!("{}", e);
                }
            },
            "exit" => {
                return
            },
            command => {
            let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();

            let _ = child.wait();
        }

    }
}
}

pub fn init_shell() {
    unish_loop();
}