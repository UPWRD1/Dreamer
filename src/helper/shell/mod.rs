/// Primary Logic for the Shell Interceptor
use super::{
    clear_term,
    colored::Colorize,
    resource::{quit, read_file_gpath},
    PromptItemKind, UserConfig, SELF_VERSION,
};

use std::{
    env::{self},
    fs::File,
    io::{stdin, stdout, BufReader, Write},
    path::Path,
    process::Child,
    process::{Command, Stdio},
    str::FromStr,
};

fn zzzsh_update_prompt(home_dir: &Result<String, env::VarError>) -> String {
    let mut future_prompt: Vec<String> = vec![];
    let config_path: String;
    if cfg!(windows) {
        config_path = format!("{}\\.snooze\\profiles\\{}\\cfg", home_dir.as_ref().unwrap(), env::var("USERNAME").unwrap());
    } else {
        config_path = format!("{}\\.snooze\\profiles\\{}\\cfg", home_dir.as_ref().unwrap(), env::var("USER").unwrap());
    }

    if let Ok(v_file) = read_file_gpath(&config_path) {
        let reader: BufReader<File> = BufReader::new(v_file.0);
        let config: Result<UserConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
        match config {
            Err(e) => {
                println!("{e}");
                quit(1);
            }

            Ok(config) => {
                let x = config.user.prompt;
                for i in x {
                    let mut ivchars: Vec<char> = i.chars().collect();
                    if (ivchars[0] == '%') && (ivchars[ivchars.len() - 1] == '%') {
                        ivchars.remove(0);
                        ivchars.remove(ivchars.len() - 1);

                        let sivchars: String = ivchars.into_iter().collect();
                        let sivcharsenum = PromptItemKind::from_str(&sivchars).expect("er");
                        match sivcharsenum {
                            PromptItemKind::HOMEDIR => {
                                future_prompt.push(home_dir.as_ref().unwrap().to_string())
                            }
                            PromptItemKind::CURRDIR => {
                                let parent = env::current_dir()
                                    .unwrap()
                                    .as_path()
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                let name = env::current_dir()
                                    .unwrap()
                                    .as_os_str()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                let mut fname = name.replace(&parent, "");
                                fname.remove(0);
                                future_prompt.push(fname);
                            }
                            PromptItemKind::USRNAME => {
                                future_prompt.push(env::var("USERNAME").unwrap())
                            }
                        }
                    } else {
                        future_prompt.push(i);
                    }
                }
            }
        }
    } else {
        future_prompt.push("$".to_string());
    };
    //println!("{}", future_prompt.join(" "));
    future_prompt.join("")
}

fn zzzsh_check_is_local(cmd: &str, env_cmds: &[String]) -> bool {
    env_cmds.contains(&cmd.to_string())
}

fn zzsh_loop(env_cmds: Vec<String>, home_dir: Result<String, env::VarError>, hashname: u64) {
    loop {
        let u_prompt: String = zzzsh_update_prompt(&home_dir);
        shellprint!("{}", u_prompt);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_cmd: Option<Child> = Default::default();

        while let Some(command) = commands.next() {
            let mut parts = command.split_whitespace(); //.map(str::to_string).collect();
            if let Some(command) = parts.next() {
                let args = parts;
                //zzzsh_check_builtin(command, args, previous_cmd, commands);
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
                        if zzzsh_check_is_local(command, &env_cmds) {
                            //println!("LOCAL");
                            //zzzsh_exec(command, args, previous_cmd, commands);
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
                            //println!("{}", cmd_local);
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
    infoprint!("Dreamer {0} (type 'exit()' to exit)", SELF_VERSION);
    zzsh_loop(env_cmds, home_dir, hashname);
}
