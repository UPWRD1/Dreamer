/// Primary Logic for the Shell Interceptor
use super::{clear_term, colored::Colorize, resource::quit, SELF_VERSION};

use std::{
    env::{self},
    io::{stdin, stdout, Write},
    path::Path,
    //str::SplitWhitespace,
    process::Child,
    process::{Command, Stdio},
};

/*
fn unish_exec(command: &str, args: SplitWhitespace<'_>, previous_cmd: &mut Option<Child>, commands: &mut std::iter::Peekable<std::str::Split<'_, &str>>) {

    let stdin = previous_cmd
    .map_or(Stdio::inherit(), |output: Child| { Stdio::from(output.stdout.unwrap())});

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
        Ok(output) => { *previous_cmd = Some(output);},
        Err(e) => {
            let previous_cmd: &mut Option<Child> = &mut None;
            errprint!("{}", e); }
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

fn unish_check_is_local(cmd: &str, env_cmds: &[String]) -> bool {
    env_cmds.contains(&cmd.to_string())
}

fn unish_loop(env_cmds: Vec<String>, home_dir: Result<String, env::VarError>, hashname: u64) {
    loop {
        let curr_dir = env::current_dir();
        shellprint!("(~{}) [zzz] @> ", curr_dir.unwrap().to_string_lossy());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_cmd: Option<Child> = Default::default();

        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace(); //.map(str::to_string).collect();
            if let Some(command) = parts.next() {
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
                    "exit()" => {
                        quit(0);
                    }

                    "clear" | "cls" => clear_term(),

                    command => {
                        if unish_check_is_local(command, &env_cmds) {
                            println!("LOCAL");
                            //unish_exec(command, args, previous_cmd, commands);
                            let stdin = previous_cmd.map_or(Stdio::inherit(), |output: Child| {
                                Stdio::from(output.stdout.unwrap())
                            });

                            let stdout = if commands.peek().is_some() {
                                Stdio::piped()
                            } else {
                                Stdio::inherit()
                            };

                            let cmd_local = format!(
                                "{0}/.snooze/bins/{1}/{2}",
                                home_dir.clone().unwrap(),
                                hashname,
                                command
                            );
                            println!("{}", cmd_local);
                            let output = Command::new(cmd_local)
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
        }
        if let Some(mut final_command) = previous_cmd {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}
pub fn init_shell(env_cmds: Vec<String>, home_dir: Result<String, env::VarError>, hashname: u64) {
    infoprint!("Counting Sheep...");
    //pause();
    //clear_term();
    infoprint!("Snooze {0} (type 'exit()' to exit)", SELF_VERSION);
    unish_loop(env_cmds, home_dir, hashname);
}
