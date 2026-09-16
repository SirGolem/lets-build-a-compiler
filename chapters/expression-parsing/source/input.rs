use crate::{
    output::{abort, expected},
    state::State,
};
use std::io::{Read, stdin};

pub fn check_character(character: char, state: &mut State) -> bool {
    match (character, state.character) {
        | (_, None) => false,
        | (expected, Some(found)) if expected == found => true,
        | (_, Some(_)) => false,
    }
}

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

pub fn is_additive_operator<Character: Into<Option<char>>>(character: Character) -> bool {
    let character = character.into();
    if let Some(character) = character { character == '+' || character == '-' } else { false }
}

pub fn is_multiplicative_operator<Character: Into<Option<char>>>(character: Character) -> bool {
    let character = character.into();
    if let Some(character) = character { character == '/' || character == '*' } else { false }
}

pub fn is_numeric<Character: Into<Option<char>>>(character: Character) -> bool {
    let character = character.into();
    if let Some(character) = character { character >= '0' && character <= '9' } else { false }
}

pub fn match_character(character: char, state: &mut State) -> () {
    match (character, state.character) {
        | (_, None) => expected(format!("'{character}'"), "end of input"),
        | (expected, Some(found)) if expected == found => get_character(state),
        | (expected, Some(found)) => crate::output::expected(format!("'{expected}'"), format!("'{found}'")),
    };
}
