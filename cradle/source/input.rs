use crate::{
    output::{abort, expected},
    state::State,
};
use std::io::{Read, stdin};

pub fn get_character(state: &mut State) -> () {
    let mut reader: Box<dyn Read> = match &state.input_file {
        | None => Box::new(stdin()),
        | Some(file) => Box::new(file),
    };

    let mut character = [0; 1];
    match reader.read(&mut character) {
        | Err(error) => abort(format!("Failed to read character: {error}")),
        | Ok(0) => state.character = None,
        | Ok(1) => state.character = Some(character[0] as char),
        | Ok(_) => abort("Failed to read character: too many bytes."),
    }
}

pub fn is_alphabetic(character: char) -> bool {
    (character >= 'A' && character <= 'Z') || (character >= 'a' && character <= 'z')
}

pub fn is_numeric(character: char) -> bool {
    character >= '0' && character <= '9'
}

pub fn match_character(character: char, state: &mut State) -> () {
    match (character, state.character) {
        | (_, None) => expected(format!("'{character}'"), "end of file"),
        | (expected, Some(found)) if expected == found => get_character(state),
        | (expected, Some(found)) => crate::output::expected(format!("'{expected}'"), format!("'{found}'")),
    };
}
