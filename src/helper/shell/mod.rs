//use super::resource::throw_fatal;
use crate::helper::{colored::Colorize, resource::clear_term};
//use crate::helper::input_fmt;
use std::{
    env::{self},
    io::{stdin, stdout, Write},
    path::Path,
    //str::SplitWhitespace,
    process::Child,
    process::{Command, Stdio},
};

use super::resource::quit;

/* 
fn unish_exec(command: &str, args: SplitWhitespace<'_>, mut previous_cmd: Option<Child>, mut commands: std::iter::Peekable<std::str::Split<'_, &str>>) {

            //unish_exec(command, args, previous_cmd, commands);
            let stdin = previous_cmd.map_or(Stdio::inherit(), |output: Child| {
                Stdio::from(output.stdout.unwrap())
            });

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let output = Command::new(command)
                .args(args)
                .stdin(stdin)
                .stdout(stdout)
                .spawn();

            match output {
                Ok(output) => {
                    return previous_cmd = Some(output);
                }
                Err(e) => {
                    errprint!("{}", e);
                    return previous_cmd = None;
                    
                }
            }
}

fn unish_check_builtin(command:&str , args: SplitWhitespace<'_>, previous_cmd: Option<Child>, commands: std::iter::Peekable<std::str::Split<'_, &str>>) {
    match command {
        "cd" => {
            let new_dir = &args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(root) {
                errprint!("{}", e);
            }
        }
        "exit" => {
            quit();
        }

        "clear" | "cls" => print!("\x1B[2J\x1B[1;1H"),

        command => {
            unish_exec(command, args, previous_cmd, commands);
        }
    }
}

*/

fn unish_check_is_local(cmd: &str) -> bool {
    super::refs::ENV_COMMANDS.contains(&cmd.to_string())
}

fn unish_loop() {
    loop {
        let curr_dir = env::current_dir();
        shellprint!("(~{}) [unify] @> ", curr_dir.unwrap().to_string_lossy());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_cmd: Option<Child> = Default::default();

        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace(); //.map(str::to_string).collect();
            match parts.next() {
                Some(command) => {
                    let args = parts;
                    //unish_check_builtin(command, args, previous_cmd, commands);
                    match command {
                        "cd" => {
                            let new_dir = &args.peekable().peek().map_or("/", |x| *x);
                            let root = Path::new(new_dir);
                            if let Err(e) = env::set_current_dir(root) {
                                errprint!("{}", e);
                            }
                        }
                        "exit" => {
                            quit();

                        }

                        "clear" | "cls" => clear_term(),

                        command => {
                            if unish_check_is_local(command) {
                                infoprint!("LOCAL");
                                //unish_exec(command, args, previous_cmd, commands);
                                let stdin = previous_cmd
                                    .map_or(Stdio::inherit(), |output: Child| {
                                        Stdio::from(output.stdout.unwrap())
                                    });

                                let stdout = if commands.peek().is_some() {
                                    Stdio::piped()
                                } else {
                                    Stdio::inherit()
                                };

                                let output = Command::new(command)
                                    .args(args)
                                    .stdin(stdin)
                                    .stdout(stdout)
                                    .spawn();

                                match output {
                                    Ok(output) => {
                                        previous_cmd = Some(output);
                                    }
                                    Err(e) => {
                                        previous_cmd = None;
                                        errprint!("{}", e);
                                    }
                                }
                            } else {
                                //unish_exec(command, args, previous_cmd, commands);
                                let stdin = previous_cmd
                                    .map_or(Stdio::inherit(), |output: Child| {
                                        Stdio::from(output.stdout.unwrap())
                                    });

                                let stdout = if commands.peek().is_some() {
                                    Stdio::piped()
                                } else {
                                    Stdio::inherit()
                                };

                                let output = Command::new(command)
                                    .args(args)
                                    .stdin(stdin)
                                    .stdout(stdout)
                                    .spawn();

                                match output {
                                    Ok(output) => {
                                        previous_cmd = Some(output);
                                    }
                                    Err(e) => {
                                        previous_cmd = None;
                                        errprint!("{}", e);
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    errprint!("No Command!");
                    quit()
                }
            }
            
        }

        if let Some(mut final_command) = previous_cmd {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}

pub fn init_shell() {
    unish_loop();
}
