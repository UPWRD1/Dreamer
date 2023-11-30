/// Command Execution
// Local imports
use crate::{
    helper::{
        check_arg_len,
        colored::Colorize,
        read_file,
        resource::{calculate_hash, continue_prompt, input_fmt, read_file_gpath, verbose_check},
        run, usage_and_quit, ConfigTool, ConfigToolInstallMethod, ZzzConfig,
    },
    STARTCMD,
};

use crate::helper::errors::*;

// std imports
use std::{
    env::{self},
    error::Error,
    fs,
    fs::File,
    io::{BufReader, Write},
    process::Command,
};

use super::{
    refs::CLEAN_FLAG,
    resource::{quit, read_file_gpath_no_f},
    ToolVec, ZzzCacheFile,
};

pub fn read_config(filepath: &String) -> Result<ZzzConfig, String> {
    match read_file_gpath_no_f(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            match config {
                Err(_) => {
                    MISSINGFILEERROR.show_error(filepath);
                    Err("Invalid Config".into())
                }
                Ok(con) => Ok(con),
            }
        }
        Err(..) => {
            MISSINGFILEERROR.show_error(filepath);
            Err("Invalid Config".into())
        }
    }
}

pub fn read_cache(filepath: &String) -> Result<ZzzCacheFile, String> {
    match read_file_gpath_no_f(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzCacheFile, serde_yaml::Error> =
                serde_yaml::from_reader(&v_file.0);
            match config {
                Err(e) => {
                    eprintln!("{}", e);
                    MISSINGFILEERROR.show_error(filepath);
                    Err("Invalid Config".into())
                }
                Ok(con) => Ok(con),
            }
        }
        Err(e) => {
            println!("{}\n\n{}", e.0, e.1);
            MISSINGFILEERROR.show_error(filepath);
            Err("Invalid Config".into())
        }
    }
}

pub fn list_exec(v_file: File, filepath: String, way: usize) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            MISSINGFILEERROR.show_error(&filepath);
            Err("Invalid Config".into())
        }

        Ok(config) => match way {
            1 => {
                if config.DEPENDANCIES.TOOLS.is_empty() {
                    warnprint!("Dream: '{}' has no dependancies!", filepath);
                    Ok(())
                } else {
                    infoprint!("'{}' requires the following dependancies:", filepath);
                    let mut num = 1;
                    for tool in config.DEPENDANCIES.TOOLS {
                        println!("\t{0}: {1} \t (from {2})", num, tool.NAME, tool.LINK);
                        num += 1;
                    }
                    Ok(())
                }
            }

            2 => {
                infoprint!("Dependancies for {}:", filepath);
                for tool in config.DEPENDANCIES.TOOLS {
                    println!("\t- {}", tool.NAME);
                }
                Ok(())
            }

            _ => {
                infoprint!("Dependancies for {}:", filepath);
                let mut num = 1;
                for tool in config.DEPENDANCIES.TOOLS {
                    println!("\t{0}: {1}", num, tool.NAME);
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
    method: ConfigToolInstallMethod,
) -> Result<(), Box<dyn Error>> {
    let link = questionprint!("Enter link for '{}':", depname);
    match read_file_gpath(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            let mut conf_f = config.unwrap();
            let n_tool: ConfigTool = ConfigTool {
                NAME: depname.to_string(),
                LINK: link,
                METHOD: method,
            };
            let mut tool_to_add: Vec<ConfigTool> = vec![n_tool];
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
            MISSINGFILEERROR.show_error(&file.1);
        }
    };

    successprint!("'{0}' added to {1}", depname, &filepath);

    Ok(())
}

pub fn remove_exec(filepath: &String, depname: &String) -> Result<(), Box<dyn Error>> {
    match read_file_gpath(filepath) {
        Ok(v_file) => {
            let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(&v_file.0);
            let mut conf_f = config.unwrap();
            let toollist = &mut conf_f.DEPENDANCIES.TOOLS;
            let index = toollist.iter().position(|x| x.NAME == *depname).unwrap();
            warnprint!(
                "This will remove {} from {}",
                toollist[index].NAME,
                filepath
            );
            continue_prompt();
            toollist.remove(index);
            conf_f.PROJECT.IS_LOADED = false;
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(v_file.1)
                .expect("Couldn't open file");
            serde_yaml::to_writer(f, &conf_f).unwrap();
        }
        Err(file) => {
            MISSINGFILEERROR.show_error(&file.1);
        }
    };

    successprint!("'{0}' removed from {1}", depname, &filepath);

    Ok(())
}

pub fn load_exec(
    root_config_file: File,
    root_config_filepath: String,
    env_cmds: Vec<String>,
    mut home_dir: Result<String, env::VarError>,
    dep_vec: ToolVec,
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(root_config_file);
    // Parse the YAML into DepConfig struct
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            MISSINGFILEERROR.show_error(&root_config_filepath);
            Err("Invalid Config".into())
        }
        Ok(config) => load_conf(
            &config,
            &root_config_filepath,
            env_cmds,
            &mut home_dir,
            dep_vec,
        ),
    }
}

fn dedup<T: Eq + std::hash::Hash + Clone>(v: &mut Vec<T>) { // note the Copy constraint
    let mut uniques = std::collections::HashSet::new();
    v.retain(|e| uniques.insert(e.clone()));
}


fn load_conf(
    config: &ZzzConfig,
    root_config_filepath: &String,
    mut env_cmds: Vec<String>,
    home_dir: &mut Result<String, env::VarError>,
    mut dep_vec: ToolVec,
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    let config_hash = calculate_hash(config);
    let mut proj_conf = config.clone();
    if !proj_conf.PROJECT.IS_LOADED || CLEAN_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
        let cachefile_path = format!(
            "{}/.snooze/cache/cache.zzz.yaml",
            home_dir.as_deref().unwrap()
        );
        
        for i in &proj_conf.DEPENDANCIES.TOOLS {
            dep_vec = search_config(&cachefile_path, i, dep_vec)
        }

        for i in dep_vec.items {
            let _ = tool_install(&i, config_hash, &mut env_cmds, home_dir);
        }
        proj_conf.PROJECT.IS_LOADED = true;
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(root_config_filepath)
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &proj_conf).unwrap();
    }
    let result = (env_cmds, config_hash);
    Ok(result)
}

fn search_config(cachefile_path: &String, to_find: &ConfigTool, mut dep_vec: ToolVec) -> ToolVec {
    match read_cache(cachefile_path) {
        Ok(cachefile) => {
                if let Some(tool) = cachefile.CACHE.iter().find(|x| x.PACKAGE == *to_find) {
                    for j in tool.clone().REQUIRES {
                        dep_vec.items.push(j.PACKAGE);
                    }
                    //println!("{} found!", tool.PACKAGE.NAME);
                    dep_vec.items.push(tool.PACKAGE.clone());
                } else {
                    errprint!("Tool '{}' not found!", to_find.NAME);
                }
                dep_vec.items.sort_unstable();
                dedup(&mut dep_vec.items);
                dep_vec
            }
            
        Err(..) => {quit(4); return dep_vec},
    }
}

pub fn load_start(
    argsv: Vec<String>,
    env_cmds: &[String],
    home_dir: Result<String, env::VarError>,
) -> Result<(Vec<String>, u64), Box<dyn Error>> {
    if check_arg_len(argsv.clone(), 2) {
        usage_and_quit(STARTCMD.name, "Missing Filename!");
        return Err("Bad File".into());
    } else {
        let _: Result<(Vec<String>, u64), ()> = match read_file(&argsv, 2, STARTCMD) {
            Ok(v_config_file) => {
                let dep_vec: ToolVec = ToolVec { items: vec![] };
                infoprint!("Starting...");
                let result: Result<(Vec<String>, u64), Box<dyn Error>> =
                    load_exec(v_config_file.0, v_config_file.1, env_cmds.to_vec(), home_dir, dep_vec);

                return result;
            }
            Err(file) => {
                MISSINGFILEERROR.show_error(&file.1);
                Err(())
            }
        };
    }
    Err("Bad File".into())
}

fn tool_install(
    tool: &ConfigTool,
    hashname: u64,
    env_cmds: &mut Vec<String>,
    home_dir: &mut Result<String, env::VarError>,
) -> Result<(), Box<dyn Error>> {
    env_cmds.push(tool.NAME.clone());
    verbose!("Installing {0} from {1}", tool.NAME, tool.LINK);
    let link = &tool.LINK;
    let method = &tool.METHOD;
    let link_str = link.to_string();
    match method {
        ConfigToolInstallMethod::LINKZIP => {
            if cfg!(windows) {
                windows_link_install(home_dir, hashname, tool, &link_str)
            } else {
                unix_link_install(home_dir, hashname, &link_str, tool)
            }
        }
        &ConfigToolInstallMethod::GIT => {
            if cfg!(windows) {
                windows_git_install(home_dir, hashname, tool, &link_str)
            } else {
                unix_git_install(home_dir, hashname, tool, link_str)
            }
        }

        &ConfigToolInstallMethod::ROOT => Ok(()),
    }
}

fn unix_link_install(
    home_dir: &mut Result<String, env::VarError>,
    hashname: u64,
    link_str: &String,
    tool: &ConfigTool,
) -> Result<(), Box<dyn Error>> {
    let dir_loc = format!(
        "{0}/.snooze/bins/{1}/",
        home_dir.as_mut().unwrap(),
        hashname
    );
    match fs::create_dir_all(&dir_loc) {
        Ok(..) => {
            let link_str_f = link_str.to_string();
            let namef = format!("{0}{1}", dir_loc, tool.NAME);
            let args: Vec<&str> = vec!["-c", "/usr/bin/curl", &link_str_f, "--output", &namef];
            let status = Command::new("bash").args(args).status()?;

            if status.success() {
                let args2: Vec<&str> = vec!["-c", "chmod", "a+x", &namef];
                let status2 = Command::new("bash").args(args2).status()?;
                if status2.success() {
                    verbose!("'{}' installed", tool.NAME);
                    Ok(())
                } else {
                    errprint!("Error grabbing: '{}'", tool.NAME);
                    Err("Error grabbing".into())
                }
            } else {
                errprint!("Error grabbing: '{}'", tool.NAME);
                Err("Error grabbing".into())
            }
        }
        Err(..) => {
            errprint!("Error creating dir");
            Err("Error creating dir".into())
        }
    }
}

fn windows_link_install(
    home_dir: &mut Result<String, env::VarError>,
    hashname: u64,
    tool: &ConfigTool,
    link_str: &String,
) -> Result<(), Box<dyn Error>> {
    let dir_loc = format!(
        "{0}\\.snooze\\bins\\{1}\\",
        home_dir.as_mut().unwrap(),
        hashname
    );
    match fs::create_dir_all(&dir_loc) {
        Ok(..) => {
            let namef = format!("{0}{1}", dir_loc, tool.NAME);
            let args: Vec<&str> = vec!["/C", "curl", link_str, "--output", &namef, "--silent"];
            let status = Command::new("cmd").args(args).status()?;
            if status.success() {
                let args2: Vec<&str> = vec!["/C", "chmod", "a+x", &namef];
                let status2 = Command::new("cmd").args(args2).status()?;
                if status2.success() {
                    verbose!("'{}' installed", tool.NAME);
                    Ok(())
                } else {
                    errprint!("Error grabbing: '{}'", tool.NAME);
                    continue_prompt();
                    Err("Error grabbing".into())
                }
                //infoprint!("Command '{}' executed successfully", command);
            } else {
                errprint!("Error grabbing: '{}'", tool.NAME);
                continue_prompt();
                Err("Error grabbing".into())
            }
        }
        Err(..) => {
            errprint!("Error creating dir");
            Err("Error creating dir".into())
        }
    }
}

fn unix_git_install(
    home_dir: &mut Result<String, env::VarError>,
    hashname: u64,
    tool: &ConfigTool,
    link_str: String,
) -> Result<(), Box<dyn Error>> {
    let dir_loc = format!(
        "{0}/.snooze/bins/{1}/",
        home_dir.as_mut().unwrap(),
        hashname
    );
    let dir_temp = format!(
        "{0}/.snooze/ztemp/{1}/",
        home_dir.as_mut().unwrap(),
        hashname
    );
    match fs::create_dir_all(&dir_loc) {
        Ok(..) => {
            let curr_dir = env::current_dir().unwrap();
            let _namef = format!("{0}{1}", dir_loc, tool.NAME);
            let args: Vec<&str> = vec!["clone", &link_str, &dir_temp, "-q"];
            verbose!("Cloning...");
            let status = Command::new("git").args(args).status()?;
            if status.success() {
                match env::set_current_dir(&dir_temp) {
                    Ok(()) => {
                        let argsv: Vec<String> =
                            vec!["".to_string(), "".to_string(), "dream".to_string()];
                        let _: Result<(), Box<dyn Error>> = match read_file(&argsv, 2, STARTCMD) {
                            Ok(v_file) => {
                                let reader: BufReader<File> = BufReader::new(v_file.0);

                                let nconfig: Result<ZzzConfig, serde_yaml::Error> =
                                    serde_yaml::from_reader(reader);
                                let synthargs: Vec<String> =
                                    vec!["zzz".to_string(), "run".to_string(), "dream".to_string()];
                                let _ = run(synthargs);
                                if status.success() {
                                    let package = &nconfig.unwrap().PROJECT.PACKAGE;
                                    let install_loc = format!(
                                        "{}/.snooze/bins/{}/",
                                        home_dir.as_ref().unwrap(),
                                        hashname
                                    );
                                    let args: Vec<&str> = vec![package, &install_loc];
                                    let ctemp = env::current_dir().unwrap();
                                    let status = Command::new("cp").args(args).status()?;
                                    if status.success() {
                                        env::set_current_dir(curr_dir)?;
                                        fs::remove_dir_all(ctemp)?;
                                        Ok(())
                                    } else {
                                        quit(4);
                                        Err("asdf".into())
                                    }
                                } else {
                                    quit(4);
                                    Err("asdf".into())
                                }
                            }
                            Err(file) => {
                                MISSINGFILEERROR.show_error(&file.1);
                                Err("Missing File".into())
                            }
                        };
                        Ok(())
                    }
                    Err(..) => {
                        quit(4);
                        Err("asdf".into())
                    }
                }
            } else {
                errprint!("Error grabbing: '{}'", tool.NAME);
                continue_prompt();
                Err("Error grabbing".into())
            }
        }
        Err(..) => {
            errprint!("Error creating dir");
            Err("Error creating dir".into())
        }
    }
}

fn windows_git_install(
    home_dir: &mut Result<String, env::VarError>,
    hashname: u64,
    tool: &ConfigTool,
    link_str: &String,
) -> Result<(), Box<dyn Error>> {
    let dir_loc = format!(
        "{0}\\.snooze\\bins\\{1}\\",
        home_dir.as_mut().unwrap(),
        hashname
    );
    let dir_temp = format!(
        "{0}\\.snooze\\ztemp\\{1}\\",
        home_dir.as_mut().unwrap(),
        hashname
    );
    match fs::create_dir_all(&dir_loc) {
        Ok(..) => {
            let curr_dir = env::current_dir().unwrap();
            let _namef = format!("{0}{1}", dir_loc, tool.NAME);
            let args: Vec<&str> = vec!["/C", "git", "clone", link_str, &dir_temp, "-q"];
            verbose!("Cloning...");
            let status = Command::new("cmd").args(args).status()?;
            if status.success() {
                match env::set_current_dir(&dir_temp) {
                    Ok(()) => {
                        let argsv: Vec<String> =
                            vec!["".to_string(), "".to_string(), "dream".to_string()];
                        let _: Result<(), Box<dyn Error>> = match read_file(&argsv, 2, STARTCMD) {
                            Ok(v_file) => {
                                let reader: BufReader<File> = BufReader::new(v_file.0);
                                let nconfig: Result<ZzzConfig, serde_yaml::Error> =
                                    serde_yaml::from_reader(reader);
                                if !nconfig
                                    .as_ref()
                                    .unwrap()
                                    .clone()
                                    .DEPENDANCIES
                                    .TOOLS
                                    .is_empty()
                                {
                                    //println!("{:?}", nconfig);
                                    /*
                                    let _ = load_conf(
                                        nconfig.as_ref().unwrap(),
                                        &root_config_filepath,
                                        env_cmds.to_vec(),
                                        home_dir,
                                    );
                                     */
                                }
                                verbose!("Building...");
                                let args: Vec<String> =
                                    vec!["zzz".into(), "run".into(), "dream".into()];
                                let status = run(args);
                                if status.is_ok() {
                                    let package = &nconfig.unwrap().PROJECT.PACKAGE;
                                    verbose!("'{package}': OK");
                                    let install_loc = format!(
                                        "{}/.snooze/bins/{}/",
                                        home_dir.as_ref().unwrap(),
                                        hashname
                                    );
                                    let args: Vec<&str> = vec!["/C", "cp", package, &install_loc];
                                    let ctemp = env::current_dir().unwrap();
                                    let status = Command::new("cmd").args(args).status()?;
                                    if status.success() {
                                        env::set_current_dir(curr_dir)?;
                                        fs::remove_dir_all(ctemp)?;
                                        Ok(())
                                    } else {
                                        quit(4);
                                        Err("asdf".into())
                                    }
                                } else {
                                    quit(4);
                                    Err("asdf".into())
                                }
                            }
                            Err(file) => {
                                MISSINGFILEERROR.show_error(&file.1);
                                Err("Missing File".into())
                            }
                        };
                        Ok(())
                    }
                    Err(..) => {
                        quit(4);
                        Err("asdf".into())
                    }
                }
            } else {
                errprint!("Error grabbing: '{}'", tool.NAME);
                continue_prompt();
                Err("Error grabbing".into())
            }
        }
        Err(..) => {
            errprint!("Error creating dir");
            Err("Error creating dir".into())
        }
    }
}

pub fn run_exec(v_file: File, filepath: String) -> Result<(), Box<dyn Error>> {
    let reader: BufReader<File> = BufReader::new(v_file);
    // Parse the YAML into PluConfig struct
    let config: Result<ZzzConfig, serde_yaml::Error> = serde_yaml::from_reader(reader);
    match config {
        Err(_) => {
            MISSINGFILEERROR.show_error(&filepath);
            Err("Invalid Config".into())
        }

        Ok(config) => {
            let mut okcount: i32 = 0;
            let mut cmdcount: i32 = 0;
            // Execute commands in the 'run' section
            verbose!("Running '{}': \n", filepath);
            for command in config.ON.RUN {
                cmdcount += 1;
                let mut parts = command.split_whitespace();
                let program = parts.next().ok_or("Missing command")?;
                let args: Vec<&str> = parts.collect();
                let status = Command::new(program).args(args).status()?;
                if status.success() {
                    okcount += 1;
                } else {
                    errprint!("Error executing command: '{}'", command);
                    continue_prompt();
                }
            }
            if cmdcount == okcount {
                verbose!("\n");
                verbose!("All tasks completed successfully");
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
        "PROJECT:
  NAME: {0}
  PACKAGE: {0}
  DESCRIPTION: ''
  VERSION: 0.0.0
  IS_LOADED: true
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
    argsv: &Vec<String>,
    home_dir: Result<String, env::VarError>,
) -> Result<(), String> {
    let mut to_exec: String;
    let argslen = &argsv.len();
    let ext_args: Vec<String>;
    if argslen < &1 {
        quit(4);
        Err("Not enough args".into())
    } else {
        match argslen {
            &1 => {
                ext_args = vec![];
            }
            _ => {
                ext_args = argsv.clone().drain(2..*argslen).collect();
            }
        }
        if cfg!(windows) {
            to_exec = format!("{}/.snooze/ext/{}.exe", home_dir.unwrap(), &argsv[1]).to_owned();
            let f_fexec = str::replace(&to_exec, "\\", "/").to_owned();
            to_exec = f_fexec.to_owned();
        } else {
            to_exec = format!("{}/.snooze/ext/{}", home_dir.unwrap(), &argsv[1].to_owned());
        }
        verbose!("Executing {}", to_exec);
        let status = Command::new(to_exec.clone()).args(ext_args).status();
        match status {
            Ok(_val) => Ok(()),
            Err(..) => {
                INVALIDEXTERROR.show_error(&to_exec);
                Err("Bad command exec".into())
            }
        }
    }
}

pub fn forget_exec(
    home_dir: Result<String, env::VarError>,
    filepath: &String,
) -> Result<(), String> {
    let config = read_config(filepath);
    match config {
        Ok(conf) => {
            let hashname = calculate_hash(&conf);
            let pathtorm = format!("{}/.snooze/bins/{}", home_dir.as_ref().unwrap(), hashname);
            let pathtorm_f = format!("~/.snooze/bins/{}", hashname);
            warnprint!("This will remove '{}'!", pathtorm_f);
            continue_prompt();
            match fs::remove_dir_all(pathtorm) {
                Ok(..) => {
                    successprint!("Forgotten");
                }
                Err(..) => {
                    errprint!("This dream hasn't been loaded, or has already been forgotten.");
                    tipprint!("No changes occurred.")
                }
            }

            Ok(())
        }
        Err(..) => {
            quit(4);
            Err("asdf".into())
        }
    }
}
