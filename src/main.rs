mod scribe_script_parser;
mod scribe_core;
mod key_translation;
mod errors;
mod misc_utils;

use clap::Parser;

use log::{debug, info, warn, error};
use env_logger;
use std::{io};
use std::io::IsTerminal;


// 
#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
#[command(about = "Program that can simulate and wait for user input, for automated tasks", long_about = None)]
struct Args {
    /// You can supply commands directly, or supply them to stdin
    commands: Option<String>,

    /// Display help for writing scribe scripts
    #[arg(long)]
    script_help: bool
}

fn print_scripting_help() -> () {
    let scripting = "\
    # TYPE command \n\
    Used for simulating user input, incl. pressing, releasing and clicking keys.\n\
    ## Usage\n\
    ```\n\
    TYPE [string|<key>|vkeyv|^key^]...;\n\
    ```\n\
    - Strings will be typed verbatim and must be in quotes/double quotes\n\
    - <key> clicks a key or a modifier\n\
    - vkeyv presses a key or a modifier\n\
    - ^key^ releases a key or a modifier\n\
    \n\
    # WAITFOR command\n\
    Halts script execution, until a specified keyboard shortcut is pressed.\n\
    \n\
    ## Usage\n\
    ```\n\
    WAITFOR string;\n\
    ```\n\
    String is a valid combination of \"modifier+modifier+...+modifier+key\"\n\
    - modifiers must be as per https://w3c.github.io/uievents-key/#keys-modifier\n\
    - key must be as per https://w3c.github.io/uievents-code/#code-value-tables\n\
    ";

    println!("{scripting}");
}


fn main() -> !{
    let exit_sucess = 0;
    let exit_failure = 1;

    let cli = Args::parse();

    if cli.script_help { 
        print_scripting_help();
        std::process::exit(exit_sucess);
    }

    env_logger::init();
    //scribe_script_parser::parse_script("./test.scribe")
    //    .expect("An error occurred while parsing or executing commands");
    
    let mut commands = String::new();
    let stdin = io::stdin(); 
    let empty_stdin = !(stdin.is_terminal());

    if empty_stdin && cli.commands == None {
        println!("A scribe script must be supplied either in stdin (e.g. via a pipe), or as a positional argument. See --help for more");
        std::process::exit(exit_failure);
    }


    std::process::exit(exit_sucess);
}
