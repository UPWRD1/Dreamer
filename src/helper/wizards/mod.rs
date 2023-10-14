
use crate::helper::{colored::Colorize, input_fmt};

use super::continue_prompt;

pub fn init_cmd_wizard() -> Result<String, ()> {
    let filename = questionprint!("Enter a name for your project:");
    infoprint!("Your file will be created as {}.uni.yaml.", filename);
    continue_prompt();
    let filename_f = format!("{}.uni.yaml", filename);
    Ok(filename_f)
}

pub fn add_cmd_wizard() {
    let filename = questionprint!("Enter a .uni.yaml file");
}