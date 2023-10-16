/// Helper functions and macros for UI, parsing and other things.
// Extern Imports
extern crate colored;
use colored::Colorize;

// Local Imports
use super::refs::{ADDCMD, HELPCMD, INITCMD, LISTCMD, LOADCMD, RUNCMD};
use crate::helper::{usage, usagenb, verbose_check, Cmd};

// std imports
use std::{
    collections::hash_map::DefaultHasher,
    env,
    fmt::Arguments,
    fs::File,
    hash::{Hash, Hasher},
    io,
    io::{BufRead, Write},
    iter::*,
};

macro_rules! errprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0} {1}","[!]".red().bold(), format_args!($($arg)*))
    }};
}

macro_rules! infoprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1}","[i]".blue().bold(), format_args!($($arg)*))
    }};
}

macro_rules! warnprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0} {1}", "[W]".yellow().bold(), format_args!($($arg)*))
    }};
}

macro_rules! successprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1}", "[✔]".green().bold(), format_args!($($arg)*))
    }};
}

pub fn read_line_expect<B: BufRead>(src: &mut B) -> io::Result<String> {
    src.lines().next().map_or(
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Input BufRead reached EOF before".to_string(),
        )),
        |line| line,
    )
}

pub fn input_fmt<B: BufRead, W: Write>(
    src: &mut B,
    dst: &mut W,
    fmt: Arguments,
) -> io::Result<String> {
    dst.write_fmt(fmt)?;
    dst.flush()?;
    read_line_expect(src)
}

macro_rules! input {
    () => {
        (read_line_expect(&mut std::io::stdin().lock()).unwrap())
    };

    ($($arg:tt)*) => {
        (input_fmt(&mut std::io::stdin().lock(), &mut std::io::stdout(), format_args!($($arg)*)).unwrap())
    };
}

macro_rules! questionprint {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        input!("    {0} {1} ", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

macro_rules! shellprint {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        print!("    {0} {1} ", "[>]".yellow().bold(), format_args!($($arg)*))
    }};
}

pub fn throw_fatal(msg: &str) {
    errprint!(
        "{0}{1}{2}",
        "FATAL ERROR: ".red().bold(),
        msg.red().bold(),
        "\t If you somehow see this, you probably need to reinstall unify, like now."
            .red()
            .bold()
    );
}

pub fn printusage(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: {0}{1}", " ./unify ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: {0}{1}", " unify ".black(), msg.black());
    }
}

pub fn hash_string(key: &String) -> String {
    let length = key.len() - 1;
    let mut hash: u128 = 2166136261;
    let key_vc: Vec<char> = key.chars().collect::<Vec<char>>();
    println!("{}", std::u128::MAX);
    for i in 0..=length {
        hash ^= key_vc[i as usize] as u128 - '0' as u128;
        println!("{hash}");
        //16777619
        //hash.overflowing_mul(rhs)
    }
    return hash.to_string();
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn printusagenb(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        println!("\t Usage: {0}{1}", " ./unify ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        println!("\t Usage: {0} {1}", " unify ".black(), msg.black());
    }
}

pub fn usage_and_quit(cmd: &str, msg: &str) {
    errprint!("{}", msg);
    usage(cmd);
    std::process::exit(0);
}

pub fn option_list(kind: &str, opts: Vec<String>, msg: &str) -> Vec<char> {
    match kind {
        "err" => {
            errprint!("{}", msg);
        }
        "info" => {
            infoprint!("{}", msg);
        }
        "warn" => {
            warnprint!("{}", msg);
        }
        &_ => {
            throw_fatal("Invalid Message Type");
        }
    }
    //let mut count = 1;
    for (i, el) in opts.iter().enumerate() {
        println!("\t  {0}: {1}", i + 1, el);
        //count += 1;
    }
    let result: String = questionprint!("==> ");
    let result_c: Vec<char> = result.chars().collect();
    //println!("{}", result_c.len());
    if result_c.len() == 1 {
        match result_c[0] {
            '1'..='9' => return result_c,
            _ => {
                quit();
            }
        }
    } else {
        quit();
    }
    result_c
}

pub fn quit() {
    std::process::exit(0);
}

pub fn clear_term() {
    print!("\x1B[2J\x1B[1;1H")
}
/*
pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "{} Press any key to continue...", "[i]".blue().bold()).unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
    print!("\n");
}
*/
pub fn printhelp(cmd: &Cmd) {
    infoprint!("{0} \t Info: {1}", cmd.name, cmd.desc);
    print!("\t");
    usagenb(cmd.name);
}

pub fn printusetemplate() {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: ./unify [--version] [--help] <command> [arguments]");
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: unify [--version] [--help] <command> [arguments]");
    }
}

fn printextrahelp(cmd: Cmd) {
    infoprint!("Help: {}", cmd.name);
    printusagenb(cmd.usage);
    println!("\t Info: {}", cmd.longdesc);
}

pub fn extrahelp(cmd: &str) {
    match matchcmd(cmd) {
        Ok(cmd) => printextrahelp(cmd),
        Err(..) => usage_and_quit(HELPCMD.name, "Invalid Command Name"),
    }
}

pub fn check_arg_len(argsv: Vec<String>, lentocheck: usize) -> bool {
    argsv.len() == lentocheck
}

pub fn matchcmd(cmd: &str) -> Result<Cmd, String> {
    match cmd {
        "help" => Ok(HELPCMD),
        "run" => Ok(RUNCMD),
        "init" => Ok(INITCMD),
        "load" => Ok(LOADCMD),
        "list" => Ok(LISTCMD),
        "add" => Ok(ADDCMD),
        &_ => Err("INVALID CMD".to_string()),
    }
}

pub fn read_file(argsv: &Vec<String>, to_open: usize) -> Result<(File, String), (String, String)> {
    if to_open < argsv.len() {
        let filepath = argsv[to_open].to_string().to_owned() + ".uni.yml";
        let file: Result<File, std::io::Error> = File::open(filepath.clone());
        match file {
            Ok(v_file) => Ok((v_file, filepath)),
            Err(_error) => {
                let filepath = argsv[to_open].to_string().to_owned() + ".uni.yaml";
                let file: Result<File, std::io::Error> = File::open(filepath.clone());
                match file {
                    Ok(v_file) => Ok((v_file, filepath)),
                    Err(error) => Err((error.to_string(), filepath)),
                }
            }
        }
    } else {
        Err((
            "Not enough Arguments!".to_string(),
            "Invalid Args".to_string(),
        ))
    }
}

pub fn read_file_gpath(filename: &String) -> Result<(File, String), (String, String)> {
        let filepath1 = filename.to_string().to_owned() + ".uni.yml";
        let file: Result<File, std::io::Error> = File::open(filepath1.clone());
        match file {
            Ok(v_file) => Ok((v_file, filepath1)),
            Err(_error) => {
                let filepath2 = filename.to_string().to_owned() + ".uni.yaml";
                let file: Result<File, std::io::Error> = File::open(filepath2.clone());
                match file {
                    Ok(v_file) => Ok((v_file, filepath2)),
                    Err(error) => Err((error.to_string(), filepath2)),
                }
            }
        }
}

pub fn print_file_list_main() -> Result<(char, Vec<String>), ()> {
    match env::current_dir() {
        Ok(dir) => {
            match crate::get_yaml_paths(dir.into_os_string().into_string().unwrap().as_str()) {
                Ok(paths) => {
                    let paths_f: Vec<String> = paths
                        .into_iter()
                        .map(|s| {
                            s.file_stem()
                                .unwrap()
                                .to_str()
                                .map(|s| s.to_string())
                                .unwrap()
                        })
                        .collect();
                    let index = option_list("info", paths_f.clone(), "Choose a file (0 to quit):");
                    let index_c = index[0];
                    if index_c.is_ascii_digit() {
                        if index_c as usize == 0 {
                            quit();
                            Err(())
                        } else {
                            Ok((index_c, paths_f))
                        }
                    } else {
                        quit();
                        Err(())
                    }
                }
                Err(..) => {
                    throw_fatal("Very bad 2");
                    Err(())
                }
            }
        }
        Err(e) => {
            throw_fatal(format!("Very Bad: {e}").as_str());
            Err(())
        }
    }
}

pub fn print_file_list() -> Result<String, ()> {
    match env::current_dir() {
        Ok(dir) => {
            match crate::get_yaml_paths(dir.into_os_string().into_string().unwrap().as_str()) {
                Ok(paths) => {
                    let paths_f: Vec<String> = paths
                        .into_iter()
                        .map(|s| {
                            s.file_stem()
                                .unwrap()
                                .to_str()
                                .map(|s| s.to_string())
                                .unwrap()
                        })
                        .collect();
                    let index = option_list("info", paths_f.clone(), "Choose a file (0 to quit):");
                    let index_c = index[0];
                    if index_c.is_ascii_digit() {
                        if index_c as usize == 0 {
                            quit();
                            Err(())
                        } else {
                            let index_u = index_c.to_digit(10).unwrap() as usize;
                            let res = paths_f[index_u - 1]
                                    .clone()
                                    .strip_suffix(".uni")
                                    .unwrap()
                                    .to_string();
                            
                            Ok(res)
                        }
                    } else {
                        quit();
                        Err(())
                    }
                }
                Err(..) => {
                    throw_fatal("Very bad 2");
                    Err(())
                }
            }
        }
        Err(e) => {
            throw_fatal(format!("Very Bad: {e}").as_str());
            Err(())
        }
    }
}

pub fn argparse(argsv: &[String], pos: usize, cmd: Cmd) -> bool {
    // Parse arguments
    cmd.aliases.contains(&argsv[pos].as_str())
}

pub fn continue_prompt() {
    match questionprint!("Do you want to continue? (Y/N)").as_str() {
        "y" | "Y" => {}
        &_ => {
            quit();
        }
    }
}

pub fn verbose_info_print(msg: String, global_opts: &[bool]) {
    if verbose_check(global_opts) {
        infoprint!("{msg}")
    }
}
