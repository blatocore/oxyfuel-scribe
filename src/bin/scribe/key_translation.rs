use global_hotkey::hotkey::HotKey;


use anyhow::{anyhow,Result};


pub fn scribe_to_enigo(input: &str) -> Result<enigo::Key> {
    match input {
        "alt"          =>  Ok(enigo::Key::Alt),
        "control" 	   =>  Ok(enigo::Key::Control),
        "shift"        =>  Ok(enigo::Key::Shift),
        "meta"         =>  Ok(enigo::Key::Meta),
        "arrow_down"   =>  Ok(enigo::Key::DownArrow),
        "arrow_left"   =>  Ok(enigo::Key::LeftArrow),
        "arrow_right"  =>  Ok(enigo::Key::RightArrow),
        "arrow_up"     =>  Ok(enigo::Key::UpArrow),
        "backspace"    =>  Ok(enigo::Key::Backspace),
        "caps_lock"    =>  Ok(enigo::Key::CapsLock),
        "delete"       =>  Ok(enigo::Key::Delete),
        "end"          =>  Ok(enigo::Key::End),
        "enter"        =>  Ok(enigo::Key::Return),
        "escape"       =>  Ok(enigo::Key::Escape),
        "home"         =>  Ok(enigo::Key::Home),
        "insert"       =>  Ok(enigo::Key::Insert),
        "page_down"    =>  Ok(enigo::Key::PageDown),
        "page_up"      =>  Ok(enigo::Key::PageUp),
        "spacebar"     =>  Ok(enigo::Key::Space),
        "tab"          =>  Ok(enigo::Key::Tab),
        _              =>  {
            let char_vec: Vec<char> = input.chars().collect();
            match char_vec.len() {
                1 => return Ok(enigo::Key::Unicode(char_vec[0])),
                _ => return Err(anyhow!("Unrecognized special character \"{}\"", input))
            }
        }
    }
}

pub fn scribe_to_global_hotkey(input: &str) -> Result<HotKey> {
    Ok(HotKey::try_from(input)?)
}

