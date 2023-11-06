use crate::helper::colored::Colorize;
use crate::helper::errors::ZzzErrorType::{Bcerr, Iferr, Mferr, Nferr};
use crate::helper::resource::*;
pub trait Printerror {
    fn show_error(&self, filename: &str, global_opts: &[bool]);
}

enum ZzzErrorType {
    Iferr,
    Mferr,
    Nferr,
    Bcerr,
    Ixerr,
}
pub struct ZzzError<'a> {
    exit_code: usize,
    message: &'a str,
    kind: ZzzErrorType,
}


pub const INVALIDFILEERR: ZzzError = ZzzError {
    exit_code: 3,
    message: "Invalid config file ",
    kind: Iferr,
};

pub const MISSINGFILEERROR: ZzzError = ZzzError {
    exit_code: 2,
    message: "Cannot find file ",
    kind: Mferr,
};

pub const NOFILESERROR: ZzzError = ZzzError {
    exit_code: 6,
    message: "There are no valid .zzz.yaml files!",
    kind: Nferr,
};

pub const BADCOMMANDERROR: ZzzError = ZzzError {
    exit_code: 6,
    message: "Failed Command: ",
    kind: Bcerr,
};

pub const INVALIDEXTERROR: ZzzError = ZzzError {
    exit_code: 6,
    message: "Invalid Extension: ",
    kind: ZzzErrorType::Ixerr,
};

impl Printerror for ZzzError<'_> {
    fn show_error(&self, filename: &str, global_opts: &[bool]) {
        match self.kind {
            Iferr => {
                let msg = format!("{}{}", self.message, filename);
                errprint!("{}", msg);
                infoprint!(
                    "Help: Try 'zzz new {}' to create a new zzz.yaml file.",
                    filename
                );
                quit(self.exit_code as i32);
            }
            Mferr => {
                errprint!("Invalid config file '{}'", filename);
    quit(2);
            }
            Nferr => {
                errprint!("{}", self.message);
                infoprint!("Help: Try 'zzz init <filename>' to create a new zzz.yaml file.");
                quit_silent(6);
            }
            Bcerr => {
                errprint!("Command failed!");
                continue_prompt(global_opts);
            }
            ZzzErrorType::Ixerr => {
                errprint!("No such extension found!");
            }
        }
    }
}
