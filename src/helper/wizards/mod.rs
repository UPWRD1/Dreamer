/// Subcommand Wizards for missing arguments.

use super::{colored::Colorize, input_fmt, resource::print_file_list, continue_prompt};

pub fn init_cmd_wizard() -> Result<String, ()> {
    let filename = questionprint!("Enter a name for your project:");
    infoprint!("Your file will be created as {}.uni.yaml.", filename);
    continue_prompt();
    let filename_f = format!("{}.uni.yaml", filename);
    Ok(filename_f)
}

pub fn add_cmd_wizard() -> Result<(String, String), ()> {
    match print_file_list() {
        Ok(res) => {
            infoprint!("Opening {}", res);
            let depname = questionprint!("Dependancy name:");
            let filename = format!("{}.uni.yaml", res);
            Ok((filename, depname))
        }
        Err(..) => {
            Err(())
        }
    }
}