// function that removes first and last character of a str
// used for unquoting etc.
// returns a tuple (surrounding character, unsurrounded string)
// does not check if length is sufficient, this check is handled by grammar
pub fn unsurround(input: &str) -> (char, String) {
    (
        input.chars().next().unwrap(),
        input.chars()
            .skip(1)
            .take(input.len() - 2)
            .collect(),
    )
}

// semi-automatically generated content below
// A lot of vim macros and sed has been involved. be cautious
pub fn print_available_keys() -> () {
    println!("Available keys. \"(W)\" = available in WAITFOR command; \"(T)\" = available in TYPE command");
    let keys = "\
    Backquote (W) (T)\n\
    Backslash (W) (T)\n\
    BracketLeft (W) (T)\n\
    BracketRight (W) (T)\n\
    Comma (W) (T)\n\
    Digit0 (W) (T)\n\
    Digit1 (W) (T)\n\
    Digit2 (W) (T)\n\
    Digit3 (W) (T)\n\
    Digit4 (W) (T)\n\
    Digit5 (W) (T)\n\
    Digit6 (W) (T)\n\
    Digit7 (W) (T)\n\
    Digit8 (W) (T)\n\
    Digit9 (W) (T)\n\
    Equal (W) (T)\n\
    IntlBackslash (W)\n\
    IntlRo (W)\n\
    IntlYen (W)\n\
    KeyA (W) (T)\n\
    KeyB (W) (T)\n\
    KeyC (W) (T)\n\
    KeyD (W) (T)\n\
    KeyE (W) (T)\n\
    KeyF (W) (T)\n\
    KeyG (W) (T)\n\
    KeyH (W) (T)\n\
    KeyI (W) (T)\n\
    KeyJ (W) (T)\n\
    KeyK (W) (T)\n\
    KeyL (W) (T)\n\
    KeyM (W) (T)\n\
    KeyN (W) (T)\n\
    KeyO (W) (T)\n\
    KeyP (W) (T)\n\
    KeyQ (W) (T)\n\
    KeyR (W) (T)\n\
    KeyS (W) (T)\n\
    KeyT (W) (T)\n\
    KeyU (W) (T)\n\
    KeyV (W) (T)\n\
    KeyW (W) (T)\n\
    KeyX (W) (T)\n\
    KeyY (W) (T)\n\
    KeyZ (W) (T)\n\
    Minus (W) (T)\n\
    Period (W) (T)\n\
    Quote (W) (T)\n\
    Semicolon (W) (T)\n\
    Slash (W) (T)\n\
    AltLeft (W)\n\
    AltRight (W)\n\
    Backspace (W) (T)\n\
    CapsLock (W) (T)\n\
    ContextMenu (W)\n\
    ControlLeft (W) (T)\n\
    ControlRight (W) (T)\n\
    Enter (W) (T)\n\
    MetaLeft (W) (T)\n\
    MetaRight (W) (T)\n\
    ShiftLeft (W) (T)\n\
    ShiftRight (W) (T)\n\
    Space (W) (T)\n\
    Tab (W) (T)\n\
    Convert (W)\n\
    KanaMode (W)\n\
    Lang1 (W)\n\
    Lang2 (W)\n\
    Lang3 (W)\n\
    Lang4 (W)\n\
    Lang5 (W)\n\
    NonConvert (W)\n\
    Delete (W) (T)\n\
    End (W) (T)\n\
    Help (W) (T)\n\
    Home (W) (T)\n\
    Insert (W) (T)\n\
    PageDown (W) (T)\n\
    PageUp (W) (T)\n\
    ArrowDown (W) (T)\n\
    ArrowLeft (W) (T)\n\
    ArrowRight (W) (T)\n\
    ArrowUp (W) (T)\n\
    NumLock (W) (T)\n\
    Numpad0 (W)\n\
    Numpad1 (W)\n\
    Numpad2 (W)\n\
    Numpad3 (W)\n\
    Numpad4 (W)\n\
    Numpad5 (W)\n\
    Numpad6 (W)\n\
    Numpad7 (W)\n\
    Numpad8 (W)\n\
    Numpad9 (W)\n\
    NumpadAdd (W)\n\
    NumpadBackspace (W)\n\
    NumpadClear (W)\n\
    NumpadClearEntry (W)\n\
    NumpadComma (W)\n\
    NumpadDecimal (W)\n\
    NumpadDivide (W)\n\
    NumpadEnter (W)\n\
    NumpadEqual (W)\n\
    NumpadHash (W)\n\
    NumpadMemoryAdd (W)\n\
    NumpadMemoryClear (W)\n\
    NumpadMemoryRecall (W)\n\
    NumpadMemoryStore (W)\n\
    NumpadMemorySubtract (W)\n\
    NumpadMultiply (W)\n\
    NumpadParenLeft (W)\n\
    NumpadParenRight (W)\n\
    NumpadStar (W)\n\
    NumpadSubtract (W)\n\
    Escape (W) (T)\n\
    Fn (W)\n\
    FnLock (W)\n\
    PrintScreen (W) (T)\n\
    ScrollLock (W) (T)\n\
    Pause (W) (T)\n\
    BrowserBack (W)\n\
    BrowserFavorites (W)\n\
    BrowserForward (W)\n\
    BrowserHome (W)\n\
    BrowserRefresh (W)\n\
    BrowserSearch (W)\n\
    BrowserStop (W)\n\
    Eject (W)\n\
    LaunchApp1 (W)\n\
    LaunchApp2 (W)\n\
    LaunchMail (W)\n\
    MediaPlayPause\n\
    MediaSelect (W)\n\
    MediaStop (W) (T)\n\
    MediaTrackNext (W) (T)\n\
    MediaTrackPrevious (W) (T)\n\
    Power (W)\n\
    Sleep (W)\n\
    AudioVolumeDown (W) (T)\n\
    AudioVolumeMute (W) (T)\n\
    AudioVolumeUp (W) (T)\n\
    WakeUp (W)\n\
    Hyper (W)\n\
    Super\n\
    Turbo (W)\n\
    Abort (W)\n\
    Resume (W)\n\
    Suspend (W)\n\
    Again (W)\n\
    Copy (W)\n\
    Cut (W)\n\
    Find (W)\n\
    Open (W)\n\
    Paste (W)\n\
    Props (W)\n\
    Select (W)\n\
    Undo (W)\n\
    Hiragana (W)\n\
    Katakana (W)\n\
    Unidentified (W)\n\
    F1 (W) (T)\n\
    F2 (W) (T)\n\
    F3 (W) (T)\n\
    F4 (W) (T)\n\
    F5 (W) (T)\n\
    F6 (W) (T)\n\
    F7 (W) (T)\n\
    F8 (W) (T)\n\
    F9 (W) (T)\n\
    F10 (W) (T)\n\
    F11 (W) (T)\n\
    F12 (W) (T)\n\
    F13 (W) (T)\n\
    F14 (W) (T)\n\
    F15 (W) (T)\n\
    F16 (W) (T)\n\
    F17 (W) (T)\n\
    F18 (W) (T)\n\
    F19 (W) (T)\n\
    F20 (W) (T)\n\
    F21 (W) (T)\n\
    F22 (W) (T)\n\
    F23 (W) (T)\n\
    F24 (W) (T)\n\
    F25 (W) (T)\n\
    F26 (W) (T)\n\
    F27 (W) (T)\n\
    F28 (W) (T)\n\
    F29 (W) (T)\n\
    F30 (W) (T)\n\
    F31 (W) (T)\n\
    F32 (W) (T)\n\
    F33 (W) (T)\n\
    F34 (W) (T)\n\
    F35 (W) (T)\n\
    BrightnessDown (W)\n\
    BrightnessUp (W)\n\
    DisplayToggleIntExt (W)\n\
    KeyboardLayoutSelect (W)\n\
    LaunchAssistant (W)\n\
    LaunchControlPanel (W)\n\
    LaunchScreenSaver (W)\n\
    MailForward (W)\n\
    MailReply (W)\n\
    MailSend (W)\n\
    MediaFastForward (W)\n\
    MediaPause (W)\n\
    MediaPlay (W)\n\
    MediaRecord (W)\n\
    MediaRewind (W)\n\
    MicrophoneMuteToggle\n\
    PrivacyScreenToggle (W)\n\
    SelectTask (W)\n\
    ShowAllWindows (W)\n\
    ZoomToggle (W)";

    println!{"{}", keys};
}



pub fn print_available_modifiers() -> () {
    println!("Available modifiers. \"(W)\" = available in WAITFOR command; \"(T)\" = available in TYPE command");
    let modifiers = "\
    Alt (W) (T)\n\
    AltGraph (W)\n\
    CapsLock (W) (T)\n\
    Control (W) (T)\n\
    Fn (W)\n\
    FnLock (W)\n\
    Meta (W) (T)\n\
    NumLock (W) (T)\n\
    ScrollLock (W) (T)\n\
    Shift (W) (T)\n\
    Symbol (W)\n\
    SymbolLock (W)";
    println!("{}", modifiers);
}
