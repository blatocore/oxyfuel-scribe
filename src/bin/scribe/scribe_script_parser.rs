use std::fs;


use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;

use log::info;

use enigo::{Enigo, Settings};


use global_hotkey::hotkey;

use anyhow::Result;

use crate::scribe_core;
use crate::key_translation::*;
use crate::misc_utils::{
    unsurround,
};

#[ derive(Parser) ]
#[ grammar = "./bin/scribe/scribe_script.pest" ]
pub struct ScribeParser;


pub fn run_from_string(script: String) -> Result<()> {
    parse_script(script)
}

pub fn run_from_file(path: &str) -> Result<()> {
    let unparsed_file = fs::read_to_string(path)?;
    parse_script(unparsed_file)
}


// Must run a script from a string, since pest can not do buffered reading
// scripts should not be too big anyway to this should not be a great issue
fn parse_script(script: String) -> Result<()> {
    let script = ScribeParser::parse(Rule::script, &script)?
        .next()
        .unwrap();

    for top_level in script.into_inner() {
        match top_level.as_rule(){
            Rule::command => {
                let mut inner_rules = top_level.into_inner();
                command_handler(&mut inner_rules)?;
            },
            _ => {}
        }
    }

    Ok(())
}

fn command_handler(r: &mut Pairs<Rule>) -> Result<()> {
    let command = r.next().unwrap();
    let mut inner_rules = command.clone().into_inner();
    match command.as_rule() {
        Rule::type_cmd    => type_cmd_handler(&mut inner_rules)?,
        Rule::waitfor_cmd => waitfor_cmd_handler(&mut inner_rules)?,
        Rule::sleep_cmd   => sleep_cmd_handler(&mut inner_rules)?,
        Rule::comment     => (),
        _ => unreachable!() 
    }

    Ok(())
}


fn waitfor_cmd_handler(r: &mut Pairs<Rule>) -> Result<()> {
    let mut hk = hotkey::HotKey::new(None, hotkey::Code::Escape);

    for trigger in r {
        match trigger.as_rule() {
            Rule::string => {
                let mut trigger_inner = String::from(trigger.as_str());
                _ = trigger_inner.pop();
                _ = trigger_inner.remove(0);

                hk = scribe_to_global_hotkey(&trigger_inner)?;
            },

            _ => unreachable!()
        };
    }
    scribe_core::wait_for_shortcut(hk)?;
    Ok(())
}

fn type_cmd_handler(r: &mut Pairs<Rule>) -> Result<()> {
    let prompt = r.next();
    if let None = prompt {
        return Ok(());
    }
    let prompt = prompt.unwrap();

    // Initialize enigo for the entire command execution
    // Pressed keys will be released after the end of command
    let mut enigo = Enigo::new(&Settings::default())?;

    for prompt_part in prompt.into_inner(){
        match prompt_part.as_rule() {
            Rule::string => {
                let (_quote, text) = unsurround(&prompt_part.as_str());
                scribe_core::type_string(&mut enigo, &text)?;
            },
            Rule::special_chr => {
                let mut inner_rules = prompt_part.into_inner();
                special_char_handler(&mut enigo, &mut inner_rules)?;
            }
            _ => unreachable!()
        }
    }

    info!("Releasing all pressed keys");

    Ok(())
}

fn sleep_cmd_handler(r: &mut Pairs<Rule>) -> Result<()> {
    // Can convert straight into u64, since grammar allows only digits => no negative values
    let amount = r.next().unwrap().as_str().parse::<u64>()?;
    scribe_core::sleep(amount);
    Ok(())
}

fn special_char_handler(en: &mut Enigo, r: &mut Pairs<Rule>) -> Result<()> {
    let special = r.next().unwrap();
    
    let key = special.clone().into_inner().next().unwrap().as_str();
    match special.as_rule() {
        Rule::click_special => scribe_core::click_special(en, key), 
        Rule::press_special => scribe_core::press_special(en, key), 
        Rule::release_special => scribe_core::release_special(en, key), 
        _ => unreachable!()
    }?;
    Ok(())
}
