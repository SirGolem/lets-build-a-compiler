use crate::{
    input::{check_character, get_character, is_additive_operator, is_multiplicative_operator, is_numeric, match_character},
    output::{emit, expected},
    state::State,
};

pub fn addition(state: &mut State) -> () {
    match_character('+', state);
    term(state);
    emit("POP R1", state);
    emit("ADD R0, R1, R0", state);
}

pub fn division(state: &mut State) -> () {
    match_character('/', state);
    factor(state);
    emit("POP R1", state);
    emit("DIVIDE R0, R1, R0", state);
}

pub fn expression(state: &mut State) -> () {
    if is_additive_operator(state.character) {
        emit("MOVE R0, #0", state);
    } else {
        term(state);
    }

    while is_additive_operator(state.character) {
        emit("PUSH R0", state);

        match state.character {
            | None => expected("an additive operator", "end of input"),
            | Some('+') => addition(state),
            | Some('-') => subtraction(state),
            | Some(character) => expected("an additive operator", format!("'{character}'")),
        }
    }
}

pub fn factor(state: &mut State) -> () {
    if check_character('(', state) {
        match_character('(', state);
        expression(state);
        match_character(')', state);
    } else {
        emit(format!("MOVE R0, #{}", integer_literal(state)), state);
    }
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

pub fn multiplication(state: &mut State) -> () {
    match_character('*', state);
    factor(state);
    emit("POP R1", state);
    emit("MULTIPLY R0, R1, R0", state);
}

pub fn subtraction(state: &mut State) -> () {
    match_character('-', state);
    term(state);
    emit("POP R1", state);
    emit("SUBTRACT R0, R1, R0", state);
}

pub fn term(state: &mut State) -> () {
    factor(state);

    while is_multiplicative_operator(state.character) {
        emit("PUSH R0", state);

        match state.character {
            | None => expected("a multiplicative operator", "end of input"),
            | Some('/') => division(state),
            | Some('*') => multiplication(state),
            | Some(character) => expected("a multiplicative operator", format!("'{character}'")),
        }
    }
}
