use crate::{
    input::{get_character, is_alphabetic, is_numeric},
    output::expected,
    state::State,
};

pub fn parse_identifier(state: &mut State) -> () {
    match state.character {
        | None => expected("an identifier", "end of file"),
        | Some(character) if is_alphabetic(character) => get_character(state),
        | Some(character) => expected("an identifier", format!("'{character}'")),
    };
}

pub fn parse_integer_literal(state: &mut State) -> () {
    match state.character {
        | None => expected("an integer literal", "end of file"),
        | Some(character) if is_numeric(character) => get_character(state),
        | Some(character) => expected("an integer literal", format!("'{character}'")),
    };
}
