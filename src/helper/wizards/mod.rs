/// Subcommand Wizards for missing arguments.

use super::{colored::Colorize, input_fmt, resource::print_file_list, continue_prompt};

use std::error::Error;

pub fn init_cmd_wizard(global_opts: &[bool]) -> String {
    let filename = questionprint!("Enter a name for your project:");
    infoprint!("Your file will be created as {}.uni.yaml.", filename);
    continue_prompt(global_opts);
    let filename_f = format!("{}.uni.yaml", filename);
    filename_f
}

pub fn add_cmd_wizard() -> Result<(String, String), Box<dyn Error>> {
    match print_file_list() {
        Ok(res) => {
            let depname = questionprint!("Dependancy name:");
            Ok((res, depname))
        }

        Err(..) => {
            Err("ERR".into())
        }
    }
}