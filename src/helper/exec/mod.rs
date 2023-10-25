/// Command Execution
// Local imports
use crate::{
    helper::{
        check_arg_len,
        colored::Colorize,
        read_file,
        resource::{calculate_hash, continue_prompt, input_fmt, read_file_gpath},
        usage_and_quit, verbose_check, verbose_info_print, Tool, ZzzConfig,
    },
    list, LOADCMD,
};

use crate::helper::errors::*;

// std imports
use std::{
    env,
    error::Error,
    fs,
    fs::File,
    io::{BufReader, Write},
    process::Command,
};

use super::resource::quit;

pub fn list_exec(
    v_file: File,
    filepath: String,
    way: usize,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            INVALIDFILEERR.show_error(&filepath, global_opts);
            Err("Invalid Config".into())
        }

        Ok(config) => match way {
            1 => {
                infoprint!("'{}' requires the following dependancies:", filepath);
                let mut num = 1;
                for tool in config.DEPENDANCIES.TOOLS {
                    println!("\t{0}: {1} \t (from {2})", num, tool.name, tool.link);
                    num += 1;
                }
                Ok(())
            }

            _ => {
                infoprint!("Dependancies for {}:", filepath);
                let mut num = 1;
                for tool in config.DEPENDANCIES.TOOLS {
                    println!("\t{0}: {1}", num, tool.name);
                    num += 1;
                }
                Ok(())
            }
        },
    }
}

pub fn add_exec(
    filepath: &String,
    depname: &String,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    let link = questionprint!("Enter link for '{}':", depname);
    match read_file_gpath(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            let mut conf_f = config.unwrap();

            let n_tool: Tool = Tool {
                name: depname.to_string(),
                link,
            };
            let mut tool_to_add: Vec<Tool> = vec![n_tool];
            //let to_w = conf_f.deps.tools.append(&mut tool_to_add);
            conf_f.DEPENDANCIES.TOOLS.append(&mut tool_to_add);
            conf_f.PROJECT.IS_LOADED = false;
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(v_file.1)
                .expect("Couldn't open file");
            serde_yaml::to_writer(f, &conf_f).unwrap();
        }
        Err(file) => {
            println!("{}.", file.0);
            MISSINGFILEERROR.show_error(&file.1, global_opts);
        }
    };

    successprint!("'{0}' added to {1}", depname, &filepath);

    Ok(())
}

pub fn remove_exec(
    filepath: &String,
    depname: &String,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    match read_file_gpath(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            let mut conf_f = config.unwrap();
            let toollist = &mut conf_f.DEPENDANCIES.TOOLS;
            let index = toollist.iter().position(|x| x.name == *depname).unwrap();
            warnprint!(
                "This will remove {} from {}",
                toollist[index].name,
                filepath
            );
            continue_prompt(global_opts);
            toollist.remove(index);
            conf_f.PROJECT.IS_LOADED = false;
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(v_file.1)
                .expect("Couldn't open file");
            serde_yaml::to_writer(f, &conf_f).unwrap();
        }
        Err(file) => {
            MISSINGFILEERROR.show_error(&file.1, global_opts);
        }
    };

    successprint!("'{0}' removed from {1}", depname, &filepath);

    Ok(())
}

pub fn load_exec(
    v_file: File,
    filepath: String,
    mut env_cmds: Vec<String>,
    mut home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
    argsv: Vec<String>,
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML into DepConfig struct
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            INVALIDFILEERR.show_error(&filepath, global_opts);
            Err("Invalid Config".into())
        }
        Ok(mut config) => {
            let hashname = calculate_hash(&config.PROJECT.NAME);
            //println!("{}", hash_string(&config.project.name));
            if !config.PROJECT.IS_LOADED {
                let _ = list(argsv.clone(), 1, global_opts);
                if global_opts[2] {
                    infoprint!("This action will download the above, and run any tasks included.");
                }
                continue_prompt(global_opts);
                infoprint!("Getting dependancies from file: '{}'", filepath);
                for tool in &config.DEPENDANCIES.TOOLS {
                    let _ = tool_install(tool, hashname, &mut env_cmds, &mut home_dir, global_opts);
                }
                config.PROJECT.IS_LOADED = true;
                let f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(filepath)
                    .expect("Couldn't open file");
                serde_yaml::to_writer(f, &config).unwrap();
            }
            let result = (env_cmds, hashname);
            Ok(result)
        }
    }
}

pub fn load_deps(
    argsv: Vec<String>,
    env_cmds: &[String],
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(LOADCMD.name, "Missing Filename!");
        return Err("Bad File".into());
    } else {
        let _: Result<(Vec<String>, u64), ()> = match read_file(&argsv, 2, LOADCMD) {
            Ok(v_file) => {
                let result = load_exec(
                    v_file.0,
                    v_file.1,
                    env_cmds.to_vec(),
                    home_dir,
                    global_opts,
                    argsv,
                );

                return result;
                //Ok(result)
            }
            Err(file) => {
                MISSINGFILEERROR.show_error(&file.1, global_opts);
                Err(())
            }
        };
    }
    Err("Bad File".into())
}

fn tool_install(
    tool: &Tool,
    hashname: u64,
    env_cmds: &mut Vec<String>,
    home_dir: &mut Result<String, env::VarError>,
    global_opts: &[bool],
) -> Result<(), Box<dyn Error>> {
    env_cmds.push(tool.name.clone());
    verbose_info_print(
        format!("Installing {0} from {1}", tool.name, tool.link),
        global_opts,
    );
    let link = &tool.link;
    let link_str = link.to_string();
    if cfg!(windows) {
        let dir_loc = format!(
            "{0}\\.snooze\\bins\\{1}\\",
            home_dir.as_mut().unwrap(),
            hashname
        );
        match fs::create_dir_all(&dir_loc) {
            Ok(..) => {
                let namef = format!("{0}{1}", dir_loc, tool.name);
                let args: Vec<&str> = vec!["/C", "curl", &link_str, "--output", &namef, "--silent"];
                //println!("{:?}", args);

                let status = Command::new("cmd").args(args).status()?;
                if status.success() {
                    let args2: Vec<&str> = vec!["/C", "chmod", "a+x", &namef];
                    let status2 = Command::new("cmd").args(args2).status()?;
                    if status2.success() {
                        verbose_info_print(format!("'{}' installed", tool.name), global_opts);
                        Ok(())
                    } else {
                        errprint!("Error grabbing: '{}'", tool.name);
                        continue_prompt(global_opts);
                        Err("Error grabbing".into())
                    }
                    //infoprint!("Command '{}' executed successfully", command);
                } else {
                    errprint!("Error grabbing: '{}'", tool.name);
                    continue_prompt(global_opts);
                    Err("Error grabbing".into())
                }
            }
            Err(..) => {
                errprint!("Error creating dir");
                Err("Error creating dir".into())
            }
        }
    } else {
        let dir_loc = format!(
            "{0}/.snooze/bins/{1}/",
            home_dir.as_mut().unwrap(),
            hashname
        );
        match fs::create_dir_all(&dir_loc) {
            Ok(..) => {
                let link_str_f = link_str.to_string();
                let namef = format!("{0}{1}", dir_loc, tool.name);
                let args: Vec<&str> = vec!["-c", "/usr/bin/curl", &link_str_f, "--output", &namef];
                let status = Command::new("bash").args(args).status()?;

                if status.success() {
                    let args2: Vec<&str> = vec!["-c", "chmod", "a+x", &namef];
                    let status2 = Command::new("bash").args(args2).status()?;
                    if status2.success() {
                        verbose_info_print(format!("'{}' installed", tool.name), global_opts);
                        Ok(())
                    } else {
                        errprint!("Error grabbing: '{}'", tool.name);
                        Err("Error grabbing".into())
                    }
                } else {
                    errprint!("Error grabbing: '{}'", tool.name);
                    Err("Error grabbing".into())
                }
            }
            Err(..) => {
                errprint!("Error creating dir");
                Err("Error creating dir".into())
            }
        }
    }
}

pub fn run_exec(
    v_file: File,
    filepath: String,
    global_opts: Vec<bool>,
) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML into PluConfig struct
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            MISSINGFILEERROR.show_error(&filepath, &global_opts);
            Err("Invalid Config".into())
        }

        Ok(config) => {
            let mut okcount: i32 = 0;
            let mut cmdcount: i32 = 0;
            // Execute commands in the 'run' section
            infoprint!("Running '{}': \n", filepath);
            for command in config.ON.RUN {
                cmdcount += 1;
                let mut parts = command.split_whitespace();
                let program = parts.next().ok_or("Missing command")?;
                let args: Vec<&str> = parts.collect();
                let status = Command::new(program).args(args).status()?;
                if status.success() {
                    if verbose_check(&global_opts) {
                        infoprint!("Command '{}' executed successfully", command);
                    }
                    okcount += 1;
                } else {
                    errprint!("Error executing command: '{}'", command);
                }
            }
            if cmdcount == okcount {
                println!();
                successprint!("All tasks completed successfully");
                println!();
            }
            Ok(())
        }
    }
}

pub fn createfile(
    ufile_name: String,
    zfile_name_f: &String,
) -> Result<std::string::String, std::string::String> {
    infoprint!("Creating file: {}", ufile_name);
    let mut ufile = File::create(&ufile_name).expect("[!] Error encountered while creating file!");
    let content = format!(
        "PROJECT: {{
  NAME: \"{}\",
  DESCRIPTION: \"\",
  VERSION: \"0.0.0\",
  IS_LOADED: false,
}}

ON:
  RUN:
    - echo hello world

DEPENDANCIES:
  TOOLS:",
        &zfile_name_f
    );
    ufile
        .write_all(content.as_bytes())
        .expect("[!] Error while writing to file");

    Ok("File Created!".to_string())
}

pub fn extension_exec(
    argsv: Vec<String>,
    home_dir: Result<String, env::VarError>,
    global_opts: &[bool],
) {
    let mut to_exec: String;
    let argslen = &argsv.len();
    let ext_args: Vec<String>;
    if argslen < &2 {
        quit(4);
    } else {
        match argslen {
            &2 => {
                ext_args = vec![];
            }
            _ => {
                ext_args = argsv.clone().drain(3..*argslen).collect();
            }
        }
        if cfg!(windows) {
            to_exec = format!("{}/.snooze/ext/{}.exe", home_dir.unwrap(), &argsv[2]).to_owned();
            let f_fexec = str::replace(&to_exec, "\\", "/").to_owned();
            to_exec = f_fexec.to_owned();
        } else {
            to_exec = argsv[2].to_owned();
        }
        verbose_info_print(format!("Executing {}", to_exec).to_string(), global_opts);
        //println!("{}", to_exec);
        //println!("{:?}", ext_args);
        let status = Command::new(to_exec.clone()).args(ext_args).status();
        match status {
            Ok(_val) => {
                //println!("OK: {}", val);
            }
            Err(..) => {
                BADCOMMANDERROR.show_error(&to_exec, global_opts);
            }
        }
    }
}
