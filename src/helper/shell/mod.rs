use std::ffi::{CStr, c_char};

use super::resource::throw_fatal;

const UNISH_BUFSIZE: usize = 64;
const UNISH_TOK_DELIM: char = ' ';

/* const UNISH_TOK_DELIM: Vec<String> = vec_of_strings![
    "\t",
    "\r",
    "\n",
    " "
];
*/

pub fn strtok<'a>(string: &'_ mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = string.find(delimiter) {
      let prefix = &string[..i];
      let suffix = &string[(i + delimiter.len_utf8())..];
      *string = suffix;
      prefix
    } else {
      let prefix = *string;
      *string = "";
      prefix
    }
  }

fn unish_splitln<'a>(line: &mut &str) -> Vec<char> {
    let mut bufsize: usize = UNISH_BUFSIZE;
    let mut position: usize = 0;
    let mut tokens: Vec<char>;
    let mut token: &char;
    // let mut tokens_backup: Vec<char>;

    if tokens.is_empty() {
        throw_fatal("Allocation Error");
    }

    token = strtok(line, UNISH_TOK_DELIM);
    while !tokens.is_empty() {
        tokens[position] = *token;
        position += 1;
        
        if position >= bufsize {
            bufsize += UNISH_BUFSIZE;
            let mut tokens_backup = tokens;
            if tokens[position] == '\0' {
                throw_fatal("Allocation Error");
            }
        }
        //token = strtok(&mut empty, UNISH_TOK_DELIM);
    }

    tokens[position] = '\0';
    return tokens

}

fn unish_loop() {
    let mut line: &mut &str;
    let mut args: Vec<char>;
    let mut status: bool;

    while status {
        //todo!();

        //line = unish_readln();
        args = unish_splitln(line);
        //status = unish_exec(args);

    }
}

fn init_shell() {
    unish_loop();
}