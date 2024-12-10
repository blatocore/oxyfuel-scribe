use std::fmt;

#[derive(Debug)]
pub struct ScribeError {
    pub kind: ScribeErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ScribeErrorKind {
    UnrecognizedChar,
    UnrecognizedKeyManipulation,
    UnknownError,
    UnableToWaitFor,
}

impl fmt::Display for ScribeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_msg = match self.kind {
            ScribeErrorKind::UnrecognizedChar => "Special character not recognized.",
            ScribeErrorKind::UnrecognizedKeyManipulation => "Unrecognized key press specification. Allowed operations are pressing: \"v enter v\", releasing: '^enter^', clicking <enter>",
            ScribeErrorKind::UnableToWaitFor => "Unable to register a keyboard shortcut to wait for" ,
            ScribeErrorKind::UnknownError => "Unknown errror.",
        };

        write!(f, "{error_msg}")
    }

}
