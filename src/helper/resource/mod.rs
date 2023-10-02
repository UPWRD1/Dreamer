/// Misc Helper functions for UI and other things.
extern crate colored;

use crate::helper::usage;
use crate::helper::usagenb;
use colored::Colorize;
use crate::helper::Cmd;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::fmt::Arguments;
use std::iter::*;

macro_rules! errprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0}  {1}","[!]".red().bold(), format_args!($($arg)*))
    }};
}

macro_rules! infoprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0}  {1}","[i]".blue().bold(), format_args!($($arg)*))
    }};
}
/*
macro_rules! warnprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0}  {1}", "[W]".yellow().bold(), format_args!($($arg)*))
    }};
}
*/

macro_rules! successprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprint!("    {0} {1}", "[✔]".green().bold(), format_args!($($arg)*))
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
        input!("    {0} {1} ", "[>]".yellow().bold(), format_args!($($arg)*))
    }};
}
/* x
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
*/

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
        infoprint!("Usage: {0} {1}", " unify ".black(), msg.black());
    }
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
/*
pub fn option_list(kind: &str, opts: Vec<&str>, msg: &str) -> std::string::String {
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
    for (i, el) in opts.iter().enumerate() {
        println!("\t\t {0}: {1}", i, el)
    }
    questionprint!("==> ")
}
*/

pub fn printhelp(cmd: Cmd) {
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

pub fn check_arg_len(argsv: Vec<String>, lentocheck: usize) -> bool {
    argsv.len() == lentocheck
}
