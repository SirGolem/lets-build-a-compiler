use crate::{
    input::{get_character, is_alphabetic, is_numeric},
    output::expected,
    state::State,
};

pub fn identifier(state: &mut State) -> char {
    match state.character {
        | None => expected("an identifier", "end of input"),
        | Some(character) if is_alphabetic(character) => {
            get_character(state);
            return character;
        }
        | Some(character) => expected("an identifier", format!("'{character}'")),
    };
}

pub fn integer_literal(state: &mut State) -> char {
    match state.character {
        | None => expected("an integer literal", "end of input"),
        | Some(character) if is_numeric(character) => {
            get_character(state);
            return character;
        }
        | Some(character) => expected("an integer literal", format!("'{character}'")),
    };
}
