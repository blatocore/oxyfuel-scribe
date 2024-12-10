use enigo::{
    Keyboard,
    Direction::{Click, Press, Release}
};

use global_hotkey::{
    hotkey, 
    GlobalHotKeyEvent,
    GlobalHotKeyManager,
};


use log::{info};

use std::time::Duration;
use std::thread;


use crate::key_translation;
use crate::errors::*;

pub fn type_string(en: &mut enigo::Enigo, prompt: &str) -> () {
    info!("typing string \"{}\"", prompt);
    let _ = en.text(prompt);
}

pub fn click_special(en: &mut enigo::Enigo, key: &str) -> Result<(), ScribeError> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("clicking \"{:?}\"", enigo_key);
    let _ = en.key(enigo_key, Click);
    Ok(())
}

pub fn press_special(en: &mut enigo::Enigo, key: &str) -> Result<(), ScribeError> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("pressing \"{:?}\"", enigo_key);
    let _ = en.key(enigo_key, Press);
    Ok(())
}

pub fn release_special(en: &mut enigo::Enigo, key: &str) -> Result<(), ScribeError> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("releasing \"{:?}\"", enigo_key);
    let _ = en.key(enigo_key, Release);
    Ok(())
}

pub fn wait_for_shortcut(hk: hotkey::HotKey) -> Result<(), ScribeError> {
    let manager = GlobalHotKeyManager::new();
    if let Err(_) = manager {
        return Err(ScribeError {
            kind: ScribeErrorKind::UnableToWaitFor,
            message: format!("unable to register a shortcut")
        });
    } 
    let manager = manager.unwrap();

    info!("registering shortcut: {:?}; key: {:?}", hk.mods, hk.key);
    let _ = manager.register(hk);

    loop {
        if let Ok(_event) = GlobalHotKeyEvent::receiver().try_recv() {
            break;
        }
    }
    info!("shortcut triggered, resuming execution");

    Ok(())
}

pub fn sleep(amount: u64) -> () {
    info!("halting execution for {amount} miliseconds");
    thread::sleep(Duration::from_millis(amount));
}
