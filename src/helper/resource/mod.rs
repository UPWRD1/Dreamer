/// Misc Helper functions for UI and other things.
extern crate colored;

use crate::helper::usage;
use crate::helper::usagenb;
use crate::helper::Cmd;
use colored::Colorize;
use std::io;
use std::io::BufRead;
use std::io::Write;
//use std::io::Read;
use std::fmt::Arguments;
use std::iter::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::File;

use super::refs::LISTCMD;
use super::refs::{HELPCMD, INITCMD, LOADCMD, RUNCMD};

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
        eprint!("    {0} {1}", "[W]".yellow().bold(), format_args!($($arg)*))
    }};
}


macro_rules! successprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0}{1}", "[✔]".green().bold(), format_args!($($arg)*))
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
        "
    If you somehow see this, you probably need to reinstall unify, like now."
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
        println!("\t\t {0}: {1}", i + 1, el);
        //count += 1;
    }
    let result: String = questionprint!("==> ");
    let result_c: Vec<char> = result.chars().collect();
    //println!("{}", result_c.len());
    if result_c.len() == 1 {
        match result_c[0] {
            '1'..='9' => {
                return result_c
            }
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
        Err(("Not enough Arguments!".to_string(), "Invalid Args".to_string() ))
    }
}
