//use super::resource::throw_fatal;
use crate::helper::colored::Colorize;
use crate::helper::input_fmt;
use std::{
    env::{self},
    path::Path,
    process::Command,
    str::SplitWhitespace,
};

//pub const WINCMDS: [&str; 4] = ["help", "ls", "cat", "clear"];

fn unish_exec(command: &str, args: SplitWhitespace<'_>) {
    let mut child = Command::new(command).args(args).spawn().unwrap();
    let _ = child.wait();
}

/*
fn unish_check_win(command: &str, args: SplitWhitespace<'_>) {
    if WINCMDS.contains(&command) {
        let mut cmdargs: Vec<&str> = vec!["-c"];
        cmdargs.push(command);
        let argsstr: Vec<&str> = args.clone().collect();
        for item in argsstr {
            cmdargs.push(item);
            infoprint!("{}", item)
        }
        infoprint!("{:?}", cmdargs);
        let mut child = Command::new("powershell").args(cmdargs).spawn().unwrap();
        let _ = child.wait().expect("failed to wait");
    } else {
        unish_exec(command, args);
    }
}
*/
fn unish_check_builtin(command: &str, args: SplitWhitespace<'_>) {
    match command {
        "cd" => {
            let new_dir = &args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(root) {
                errprint!("{}", e);
            }
        }
        "exit" => return,

        "clear" | "cls" => print!("\x1B[2J\x1B[1;1H"),

        &_ => unish_exec(command, args),
    }
}

fn unish_loop() {
    loop {
        let curr_dir = env::current_dir();
        let input = shellprint!("(~{}) [unify] @> ", curr_dir.unwrap().to_string_lossy());
        let mut parts = input.split_whitespace(); //.map(str::to_string).collect();
        let command = parts.next().unwrap();
        let args = parts;
        unish_check_builtin(command, args);
    }
}

pub fn init_shell() {
    unish_loop();
}
