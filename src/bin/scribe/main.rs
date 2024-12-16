mod scribe_script_parser;
mod scribe_core;
mod key_translation;
mod misc_utils;

use clap::Parser;

use env_logger;
use log::error;

use anyhow::Result;

use std::{io};
use std::io::{IsTerminal, Read};

#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
#[command(about = "Program that can simulate and wait for user input, for automated tasks", long_about = None)]
struct Args {
    /// Read script from file, if file is not provided, program will attempt reading from stdin
    file: Option<String>,

    /// Display help for writing scribe scripts
    #[arg(long)]
    script_help: bool,

    /// Display available key names. These names are used in individual key-presses in
    /// TYPE command and in WAITFOR command (according to
    /// https://w3c.github.io/uievents-code/##code-value-tables)
    #[arg(long)]
    keylist: bool,

    /// Display available modifier names. These names are used in individual key-presses in
    /// TYPE command and in WAITFOR command (according to
    /// https://w3c.github.io/uievents-code/##code-value-tables)
    #[arg(long)]
    modlist: bool,
}

fn print_scripting_help() -> () {
    let scripting = "\
Scripts consist of commands, separated by either ';' or '\n'. Available commands are `TYPE`, `WAITFOR`, `SLEEP`.

### TYPE command 
Used for simulating user input, incl. pressing, releasing and clicking keys.

```
TYPE [string|<key>|vkeyv|^key^]...;
```
- Strings will be typed verbatim and must be in quotes/double quotes. Only printable characters
- <key> clicks a key or a modifier
- vkeyv presses a key or a modifier
- ^key^ releases a key or a modifier

Names of individual keys that can be pressed, released, or clicked, can be found by:
```
scribe [--keylist|--modlist] | grep '(T)'
```

### WAITFOR command
Halts script execution, until a specified keyboard shortcut is pressed.

```
WAITFOR string;
```
String is a valid combination of \"modifier+modifier+...+modifier+key\"
- modifiers must be as per https://w3c.github.io/uievents-key/#keys-modifier
- key must be as per https://w3c.github.io/uievents-code/#code-value-tables

Names of individual keys and modifiers that can be used to register a keyboard-shortcut can be found by:
```
scribe [--keylist|--modlist] | grep '(W)'
```

### SLEEP command
```
SLEEP <miliseconds>
```
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

    if cli.keylist {
        misc_utils::print_available_keys();
        std::process::exit(exit_sucess);
    }
    if cli.modlist {
        misc_utils::print_available_modifiers();
        std::process::exit(exit_sucess);
    }

    env_logger::init();
    //scribe_script_parser::parse_script("./test.scribe")
    //    .expect("An error occurred while parsing or executing commands");
    
    let empty_stdin = io::stdin().is_terminal();

    if empty_stdin && cli.file == None {
        println!("A scribe script file must be supplied, or provided in stdin (e.g via a pipe). See --help for more");
        std::process::exit(exit_failure);
    }

    let err: Result<()>;

    if let Some(f) = cli.file {
        err = scribe_script_parser::run_from_file(&f);
    }
    else {
        let mut script = String::new();
        let _ = io::stdin().read_to_string(&mut script);
        err = scribe_script_parser::run_from_string(script);
    }


    match err {
        Err(x) => { 
            error!("An error has occured: {}", x.to_string());
            std::process::exit(exit_failure);
        },
        Ok(_)          => std::process::exit(exit_sucess),
    }
}
