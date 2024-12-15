use enigo::{
    Keyboard,
    Direction::{Click, Press, Release}
};

use global_hotkey::{
    hotkey, 
    GlobalHotKeyEvent,
    GlobalHotKeyManager,
};

use anyhow::Result;

use log::{info};

use std::time::Duration;
use std::thread;


use crate::key_translation;

pub fn type_string(en: &mut enigo::Enigo, prompt: &str) -> Result<()> {
    info!("typing string \"{}\"", prompt);
    en.text(prompt)?;
    Ok(())
}

pub fn click_special(en: &mut enigo::Enigo, key: &str) -> Result<()> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("clicking \"{:?}\"", enigo_key);
    en.key(enigo_key, Click)?;
    Ok(())
}

pub fn press_special(en: &mut enigo::Enigo, key: &str) -> Result<()> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("pressing \"{:?}\"", enigo_key);
    en.key(enigo_key, Press)?;
    Ok(())
}

pub fn release_special(en: &mut enigo::Enigo, key: &str) -> Result<()> {
    let enigo_key = key_translation::scribe_to_enigo(key)?;
    info!("releasing \"{:?}\"", enigo_key);
    en.key(enigo_key, Release)?;
    Ok(())
}

pub fn wait_for_shortcut(hk: hotkey::HotKey) -> Result<()> {
    let manager = GlobalHotKeyManager::new()?;

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
