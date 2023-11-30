/// Helper functions and macros for UI, parsing and other things.
// Extern Imports
extern crate colored;

use crate::helper::{
    colored::Colorize,
    refs::{AVAILABLE_ARGS, CLEAN_FLAG, CLEANARG, DUMBARG, FORCEARG, VERBOSEARG},
};

// Local Imports
use super::{
    refs::{
        ADDCMD, AVAILABLE_CMDS, DUMB_FLAG, EXTCMD, FORCE_FLAG, HELPCMD, LISTCMD, NEWCMD, RUNCMD, STARTCMD,
        VERBOSE_FLAG,
    },
    ZzzConfig,
};
use crate::helper::{errors::Printerror, Cmd, PathBuf, NOFILESERROR};

// std imports
use std::{
    collections::hash_map::DefaultHasher,
    env::{self, VarError},
    error::Error,
    fmt::Arguments,
    fs,
    fs::File,
    hash::{Hash, Hasher},
    io,
    io::{BufRead, Write},
    sync::atomic::Ordering,
};

/// Print UI Error messages to stderr
macro_rules! errprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("{0} {1}","[!]".red().bold(), format_args!($($arg)*))
    }};
}
/// Print UI Info messages to stdout
macro_rules! infoprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("{0} {1}","[i]".blue().bold(), format_args!($($arg)*))
    }};
}

/// Print UI warning messages to stderr
macro_rules! warnprint {
    () => {
        eprint!("\n")
    };
    ($($arg:tt)*) => {{
        eprintln!("{0} {1}", "[w]".yellow().bold(), format_args!($($arg)*))
    }};
}

/// Print UI success messages to stdout
macro_rules! successprint {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("{0} {1}", "[✔]".green().bold(), format_args!($($arg)*))
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
        input!("{0} {1} ", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

macro_rules! questionprint_no_res {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        println!("{0} {1} ", "[?]".cyan().bold(), format_args!($($arg)*))
    }};
}

/// Wrapper for input!()
macro_rules! questionprintnof {
    () => {
        input!()
    };
    ($($arg:tt)*) => {{
        input!("{0}{1} ", "", format_args!($($arg)*))
    }};
}

macro_rules! verbose {
    ($($args:tt)*) => {
        if verbose_check() {
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
        let content = format!("{0} {1}", "[i]".black(), format_args!($($arg)*));
        println!("{}", content.black());
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
fn print_usage(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        infoprint!("Usage: {0}{1}", " ./zzz ".black(), msg.black());
    } else if ostype == "linux" || ostype == "macos" {
        infoprint!("Usage: {0}{1}", " zzz ".black(), msg.black());
    }
}

pub fn usage(cmd: &str) {
    print_usage(match_command(cmd).unwrap().usage);
}

#[deprecated]
pub fn old_calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn calculate_hash(config: &ZzzConfig) -> u64 {
    let mut hasher = DefaultHasher::new();
    let tohash = format!(
        "{}{}{}{}",
        &config.PROJECT.NAME,
        &config.PROJECT.DESCRIPTION,
        &config.PROJECT.PACKAGE,
        &config.PROJECT.VERSION
    );
    tohash.hash(&mut hasher);
    hasher.finish()
}

pub fn printusage_no_f(msg: &str) {
    let ostype = std::env::consts::OS;
    if ostype == "windows" {
        println!(
            "    {0}{1}{2}",
            "Usage: ".bold(),
            " ./zzz ".black(),
            msg.black()
        );
    } else if ostype == "linux" || ostype == "macos" {
        println!(
            "    {0}{1}{2}",
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
    for (count, item) in opts.iter().enumerate() {
        println!("\t  {0}: {1}", count + 1, item);
        //count += 1;
    }
    let result: String = questionprintnof!("==> ");
    let result_as_char_vec: Vec<char> = result.chars().collect();
    if result_as_char_vec.len() == 1 {
        match result_as_char_vec[0] {
            '1'..='9' => return result_as_char_vec,
            _ => {
                quit(0);
            }
        }
    } else {
        quit(0);
    }
    result_as_char_vec
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
    print!("    {}", "Info: \n".bold());
    let description_as_char_vec: Vec<char> = longdesc.chars().collect();
    print!("             ");
    let mut counter = 0;
    for char_item in &description_as_char_vec {
        counter += 1;
        if counter > 40 && (char_item == &' ' || char_item == &'\n') {
            println!();
            print!("             ");
            counter = 0;
        } else if char_item == &'!' {
            print!("\n\n");
        } else {
            print!("{char_item}");
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
    print!("    {}", "Aliases: ".bold());
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
        println!();
        infoprint!("Available Extensions:");
        let fhdir = format!("{}/.snooze/ext/", homedir.unwrap());
        let paths = fs::read_dir(fhdir).unwrap();

        for path in paths {
            println!("\t - {}", path.unwrap().file_name().to_str().unwrap())
        }
    } else if cmd == "args" {
    } else {
        match match_command(cmd) {
            Ok(cmd) => printextrahelp(cmd),
            Err(..) => usage_and_quit(HELPCMD.name, "Invalid Command Name"),
        }
    }
}

pub fn check_arg_len(argsv: Vec<String>, lentocheck: usize) -> bool {
    argsv.len() == lentocheck
}

pub fn match_command(cmd: &str) -> Result<Cmd, String> {
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
            Ok(ok_file) => Ok((ok_file, filepath)),
            Err(_error) => {
                let filepath = argsv[to_open].to_string().to_owned() + ".zzz.yaml";
                let file: Result<File, std::io::Error> = File::open(filepath.clone());
                match file {
                    Ok(ok_file) => Ok((ok_file, filepath)),
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
                        let paths_as_string: Vec<String> = paths
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
                            option_list("info", paths_as_string.clone(), "Choose a file (0 to quit):");
                        let index_as_char = index[0];
                        if index_as_char.is_ascii_digit() {
                            if index_as_char as usize == 0 {
                                quit(0);
                                Err("Quitted".into())
                            } else {
                                let index_as_usize = index_as_char.to_digit(10).unwrap() as usize;
                                let result_string = paths_as_string[index_as_usize - 1]
                                    .clone()
                                    .strip_suffix(".zzz")
                                    .unwrap()
                                    .to_string();
                                Ok(('!', vec![], result_string))
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

pub fn continue_prompt() {
    if FORCE_FLAG.load(Ordering::Relaxed) {
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

pub fn verbose_check() -> bool {
    VERBOSE_FLAG.load(Ordering::Relaxed)
}

pub fn verbose_set_true() {
    VERBOSE_FLAG.store(true, Ordering::Relaxed);
}

pub fn force_set_true() {
    FORCE_FLAG.store(true, Ordering::Relaxed);
}

pub fn clean_set_true() {
    CLEAN_FLAG.store(true, Ordering::Relaxed);
}

pub fn dumb_set_true() {
    DUMB_FLAG.store(true, Ordering::Relaxed);
}

pub fn scan_flags(argsv: &[String]) {
    let argsv_as_joined_string = argsv.join(" ");
    let argsv_string_as_chars = argsv_as_joined_string.chars().collect::<Vec<char>>();
    //dbg!(&argsvstring);
    if argsv_as_joined_string.contains(&"-".to_string()) {
        let flags_index = argsv_string_as_chars.iter().position(|x| x == &'-').unwrap();
        let flags_counter = &argsv_string_as_chars[flags_index + 1..];
        for char_item in flags_counter {
            for argument in AVAILABLE_ARGS {
                if argument.string_switch == char_item.to_string() {
                    match argument {
                        str if str == &VERBOSEARG => {
                            verbose_set_true();
                            break;
                        }
                        str if str == &FORCEARG => {
                            force_set_true();
                            break;
                        }
                        str if str == &CLEANARG => {
                            clean_set_true();
                            break;
                        }
                        str if str == &DUMBARG => {
                            dumb_set_true();
                            colored::control::set_override(false);
                            break;
                        }
                        &_ => {
                            break;
                        }
                    }
                }
            }
        }
    }
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
        NOFILESERROR.show_error("dummy");
        Err("No files".into())
    }
}
