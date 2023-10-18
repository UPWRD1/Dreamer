use crate::helper::resource::*;
use crate::helper::colored::Colorize;

pub fn invalid_file_error(filename: &String) {
    errprint!("Cannot find file '{}'", filename);
    infoprint!(
        "Help: Try 'unify init {}' to create a new uni.yaml file.",
        filename
    );
    quit(3);
}

pub fn missing_file_error(filename: &String) {
    errprint!("Invalid config file '{}'", filename);
    quit(2);
}

pub fn no_files_error() {
    errprint!("There are no valid .uni.yaml files!");
    infoprint!(
        "Help: Try 'unify init <filename>' to create a new uni.yaml file.");
    quit_silent(6);
}