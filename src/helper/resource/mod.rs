/// Helper functions and macros for UI, parsing and other things.
// Extern Imports
extern crate colored;

use crate::helper::colored::Colorize;

// Local Imports
use super::refs::{ADDCMD, EXTCMD, HELPCMD, LISTCMD, STARTCMD, NEWCMD, RUNCMD, AVAILABLE_CMDS};
use crate::helper::{errors::Printerror, Cmd, PathBuf, NOFILESERROR};

// std imports
use std::{
    collections::hash_map::DefaultHasher,
    env::{self, VarError},
    fs,
    error::Error,
    fmt::Arguments,
    fs::File,
    hash::{Hash, Hasher},
    io,
    io::{BufRead, Write},
    iter::*,
};

/// Print UI Error messages to stderr
macro_rules! errprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0} {1}","[!]".red().bold(), format_args!($($arg)*))
    }};
}
/// Print UI Info messages to stdout
macro_rules! infoprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1}","[i]".blue().bold(), format_args!($($arg)*))
    }};
}

/// Print UI warning messages to stderr
macro_rules! warnprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("    {0} {1}", "[w]".yellow().bold(), format_args!($($arg)*))
    }};
}

/// Print UI success messages to stdout
macro_rules! successprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1}", "[✔]".green().bold(), format_args!($($arg)*))
    }};
}

fn read_line_expect<B: BufRead>(src: &mut B) -> io::Result<String> {
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

/// Input macro to facilitate user input
macro_rules! input {
    () => {
        (read_line_expect(&mut std::io::stdin().lock()).unwrap())
    };

    ($($arg:tt)*) => {
        (input_fmt(&mut std::io::stdin().lock(), &mut std::io::stdout(), format_args!($($arg)*)).unwrap())
    };
}
/// Wrapper for input!()
macro_rules! questionprint {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        input!("    {0} {1} ", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

macro_rules! questionprint_no_res {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1} ", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

/// Wrapper for input!()
macro_rules! questionprintnof {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        input!("    {0}{1} ", "", format_args!($($arg)*))
    }};
}

macro_rules! verbose {
    ($global_opts:expr, $($args:tt)*) => {
        if verbose_check($global_opts) {
            infoprint!($($args)*);
        }
    };
}

/// Macro for printing the shell
macro_rules! shellprint {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        print!("{0} {1} ", "[>]".yellow().bold(), format_args!($($arg)*))
    }};
}

/// Print UI tip messages to stdout
macro_rules! tipprint {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        println!("    {0} {1}", "[i]".black(), format_args!($($arg)*))
    }};
}

/// Throw a fatal internal error.
pub fn throw_fatal(msg: &str) {
    errprint!(
        "{0}{1}{2}",
        "FATAL ERROR: ".red().bold(),
        msg.red().bold(),
        "\t If you somehow see this, you probably need to reinstall Dreamer, like now."
            .red()
            .bold()
    );
}

/// helper function for usage()
fn printusage(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: {0}{1}", " ./zzz ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: {0}{1}", " zzz ".black(), msg.black());
    }
}

pub fn usage(cmd: &str) {
    printusage(matchcmd(cmd).unwrap().usage);
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn printusage_no_f(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        println!(
            "\t{0}{1}{2}",
            "Usage: ".bold(),
            " ./zzz ".black(),
            msg.black()
        );
    } else if ostype == "linux" || ostype == "macos" {
        println!(
            "\t{0}{1}{2}",
            "Usage: ".bold(),
            " zzz ".black(),
            msg.black()
        );
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
    let result: String = questionprintnof!("==> ");
    let result_c: Vec<char> = result.chars().collect();
    //println!("{}", result_c.len());
    if result_c.len() == 1 {
        match result_c[0] {
            '1'..='9' => return result_c,
            _ => {
                quit(0);
            }
        }
    } else {
        quit(0);
    }
    result_c
}

pub fn quit(status: i32) {
    infoprint!("Quitting...");
    std::process::exit(status);
}

pub fn quit_silent(status: i32) {
    std::process::exit(status);
}

pub fn clear_term() {
    print!("\x1B[2J\x1B[1;1H")
}

pub fn long_infoprint(longdesc: &str) {
    print!("\t{}", "Info: \n".bold());
    let char_desc: Vec<char> = longdesc.chars().collect();
    print!("\t\t");
    let mut numchars = 0;
    for i in &char_desc {
        numchars += 1;
        if numchars > 40 && (i == &' ' || i == &'\n') {
            print!("\n\t");
            print!("        ");
            numchars = 0;
        } else if i == &'!' {
            print!("\n\n\t\t");
        } else {
            print!("{i}");
        }
    }
    println!();
}

pub fn printhelp(cmd: &Cmd) {
    println!("{}  \t{}", cmd.name, cmd.desc);
}

pub fn printusetemplate() {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("{} ./zzz [--help] <command> [arguments]\n", "Usage:".bold());
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("{} zzz [--help] <command> [arguments]\n", "Usage:".bold());
    }
}

fn printaliases(cmd: &Cmd<'_>) {
    print!("\t{}", "Aliases: ".bold());
    for i in cmd.aliases {
        print!("{}, ", i);
    }
    println!();
}

fn printextrahelp(cmd: Cmd) {
    infoprint!("{}{}", "Help:\t".bold(), cmd.name);
    println!();
    printusage_no_f(cmd.usage);
    println!();
    printaliases(&cmd);
    println!();
    long_infoprint(cmd.longdesc);
}

pub fn extrahelp(cmd: &str, homedir: Result<String, VarError>) {
    if cmd == "all" {
        infoprint!("Available Commands:");
        for x in AVAILABLE_CMDS {
            print!("\t - ");
            printhelp(x);
        }
        println!("");
        infoprint!("Available Extensions:");
        let fhdir = format!("{}/.snooze/ext/", homedir.unwrap());
        let paths = fs::read_dir(fhdir).unwrap();

        for path in paths {
            println!("\t - {}", path.unwrap().file_name().to_str().unwrap())
        }
    } else {
    match matchcmd(cmd) {
        Ok(cmd) => printextrahelp(cmd),
        Err(..) => usage_and_quit(HELPCMD.name, "Invalid Command Name"),
    }
}
}

pub fn check_arg_len(argsv: Vec<String>, lentocheck: usize) -> bool {
    argsv.len() == lentocheck
}

pub fn matchcmd(cmd: &str) -> Result<Cmd, String> {
    match cmd {
        "help" => Ok(HELPCMD),
        "run" => Ok(RUNCMD),
        "new" => Ok(NEWCMD),
        "start" => Ok(STARTCMD),
        "list" => Ok(LISTCMD),
        "add" => Ok(ADDCMD),
        "ext" => Ok(EXTCMD),
        &_ => Err("INVALID CMD".to_string()),
    }
}

pub fn read_file(
    argsv: &Vec<String>,
    to_open: usize,
    caller: Cmd,
) -> Result<(File, String), (String, String)> {
    if to_open < argsv.len() {
        let filepath = argsv[to_open].to_string().to_owned() + ".zzz.yml";
        let file: Result<File, std::io::Error> = File::open(filepath.clone());
        match file {
            Ok(v_file) => Ok((v_file, filepath)),
            Err(_error) => {
                let filepath = argsv[to_open].to_string().to_owned() + ".zzz.yaml";
                let file: Result<File, std::io::Error> = File::open(filepath.clone());
                match file {
                    Ok(v_file) => {
                        Ok((v_file, filepath))},
                    Err(error) => Err((error.to_string(), filepath)),
                }
            }
        }
    } else {
        usage_and_quit(caller.name, "Not Enough Argumets!");
        Err((
            "Not enough Arguments!".to_string(),
            "Invalid Args".to_string(),
        ))
    }
}

pub fn read_file_gpath_no_f(filename: &String) -> Result<(File, String), (String, String)> {
    let filepath1 = filename.to_string().to_owned();
    let file: Result<File, std::io::Error> = File::open(filepath1.clone());
    match file {
        Ok(v_file) => Ok((v_file, filepath1)),
        Err(error) => Err((error.to_string(), filepath1)),
    }
}

pub fn read_file_gpath(filename: &String) -> Result<(File, String), (String, String)> {
    let filepath1 = filename.to_string().to_owned() + ".zzz.yml";
    let file: Result<File, std::io::Error> = File::open(filepath1.clone());
    match file {
        Ok(v_file) => Ok((v_file, filepath1)),
        Err(_error) => {
            let filepath2 = filename.to_string().to_owned() + ".zzz.yaml";
            let file: Result<File, std::io::Error> = File::open(filepath2.clone());
            match file {
                Ok(v_file) => Ok((v_file, filepath2)),
                Err(error) => Err((error.to_string(), filepath2)),
            }
        }
    }
}

pub fn print_file_list(way: usize) -> Result<(char, Vec<String>, String), Box<dyn Error>> {
    if way == 0 {
        match env::current_dir() {
            Ok(dir) => {
                match crate::helper::resource::get_yaml_paths(
                    dir.into_os_string().into_string().unwrap().as_str(),
                ) {
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
                        let index =
                            option_list("info", paths_f.clone(), "Choose a file (0 to quit):");
                        let index_c = index[0];
                        if index_c.is_ascii_digit() {
                            if index_c as usize == 0 {
                                quit(0);
                                Err("Quitted".into())
                            } else {
                                let index_u = index_c.to_digit(10).unwrap() as usize;
                                let res = paths_f[index_u - 1]
                                    .clone()
                                    .strip_suffix(".zzz")
                                    .unwrap()
                                    .to_string();
                                println!("{res}");
                                Ok(('!', vec![], res))
                            }
                        } else {
                            quit(1);
                            Err("Not a digit".into())
                        }
                    }
                    Err(..) => {
                        throw_fatal("Very bad 2");
                        Err("very bad".into())
                    }
                }
            }
            Err(e) => {
                throw_fatal(format!("Very Bad: {e}").as_str());
                Err("oh crap".into())
            }
        }
    } else if way == 1 {
        match env::current_dir() {
            Ok(dir) => {
                match crate::helper::resource::get_yaml_paths(
                    dir.into_os_string().into_string().unwrap().as_str(),
                ) {
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
                        let index =
                            option_list("info", paths_f.clone(), "Choose a file (0 to quit):");
                        let index_c = index[0];
                        if index_c.is_ascii_digit() {
                            if index_c as usize == 0 {
                                quit(0);
                                Err("Quitted".into())
                            } else {
                                Ok((index_c, paths_f, index_c.to_string()))
                            }
                        } else {
                            quit(1);
                            Err("Not a digit".into())
                        }
                    }
                    Err(..) => {
                        throw_fatal("Very bad 2");
                        Err("Very bad".into())
                    }
                }
            }
            Err(e) => {
                throw_fatal(format!("Very Bad: {e}").as_str());
                Err("oh crap".into())
            }
        }
    } else {
        quit(4);
        Err("INVALID".into())
    }
}

pub fn argparse(argsv: &[String], pos: usize, cmd: Cmd) -> bool {
    // Parse arguments
    cmd.aliases.contains(&argsv[pos].as_str())
}

pub fn continue_prompt(global_opts: &[bool]) {
    if global_opts[1] {
    } else {
        questionprint_no_res!("Do you want to continue? (y/n)");
        match questionprintnof!("==>").as_str() {
            "y" | "Y" => {}
            &_ => {
                quit(0);
            }
        }
    }
}

pub fn verbose_info_print(msg: String, global_opts: &[bool]) {
    if verbose_check(global_opts) {
        infoprint!("{msg}")
    }
}

pub fn verbose_check(global_opts: &[bool]) -> bool {
    if !global_opts.is_empty() {
        global_opts[0]
    } else {
        false
    }
}

pub fn verbose_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-v".to_string()) {
        global_opts.insert(0, true);
        global_opts.to_vec()
    } else {
        global_opts.to_vec()
    }
}

pub fn force_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-f".to_string()) {
        global_opts.insert(1, true);
        global_opts.to_vec()
    } else {
        global_opts.to_vec()
    }
}


pub fn clean_set_true(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    if argsv.contains(&"-c".to_string()) {
        global_opts.insert(2, true);
        global_opts.to_vec()
    } else {
        global_opts.to_vec()
    }
}


pub fn scan_flags(argsv: &[String], global_opts: &mut Vec<bool>) -> Vec<bool> {
    let dream_flags: Vec<&str> = vec!["-v", "-f", "-c"];
    for i in dream_flags {
        if argsv.contains(&i.to_owned().to_string()) {
            match i {
                "-v" => {
                    verbose_set_true(argsv, global_opts);
                }

                "-f" => {
                    force_set_true(argsv, global_opts);
                }

                "-c" => {
                    clean_set_true(argsv, global_opts);
                }

                &_ => {}
            }
        }
    }
    global_opts.to_vec()
}

pub fn get_yaml_paths(dir: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than .yaml or .yml
        .filter_map(|path| {
            if path
                .extension()
                .map_or(false, |ext| (ext == "yaml") || ext == "yml")
            {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if !paths.is_empty() {
        Ok(paths)
    } else {
        let dummy: Vec<bool> = vec![false];
        NOFILESERROR.show_error("dummy", &dummy);
        Err("No files".into())
    }
}
