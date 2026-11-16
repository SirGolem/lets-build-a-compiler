use crate::{
    input::{
        check_character, get_character, is_additive_operator, is_alphabetic, is_alphanumeric, is_multiplicative_operator,
        is_numeric, match_character, skip_white_space,
    },
    output::{emit, expected},
    state::State,
};

pub fn addition(state: &mut State) -> () {
    match_character('+', state);
    term(state);
    emit("POP R1", state);
    emit("ADD R0, R1, R0", state);
}

pub fn assignment(state: &mut State) -> () {
    let name = name(state);
    match_character('=', state);
    expression(state);
    emit(format!("STORE [{name}], R0"), state);
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
    };

    while is_additive_operator(state.character) {
        emit("PUSH R0", state);

        match state.character {
            | None => expected("an additive operator", "end of input"),
            | Some('+') => addition(state),
            | Some('-') => subtraction(state),
            | Some(character) => expected("an additive operator", format!("'{character}'")),
        };
    }
}

pub fn factor(state: &mut State) -> () {
    if check_character('(', state) {
        match_character('(', state);
        expression(state);
        match_character(')', state);
    } else if is_alphabetic(state.character) {
        identifier(state);
    } else {
        emit(format!("MOVE R0, #{}", integer_literal(state)), state);
    };
}

pub fn identifier(state: &mut State) -> () {
    let name = name(state);
    if check_character('(', state) {
        match_character('(', state);
        match_character(')', state);
        emit(format!("BRANCH {name}"), state);
    } else {
        emit(format!("LOAD R0, [{name}]"), state);
    }
}

pub fn integer_literal(state: &mut State) -> String {
    let mut value = String::new();

    match state.character {
        | None => expected("an integer literal", "end of input"),
        | Some(character) if is_numeric(character) => {
            value += &character.to_string();
            get_character(state);
        }
        | Some(character) => expected("an integer literal", format!("'{character}'")),
    };

    while is_numeric(state.character) {
        value += &state.character.unwrap().to_string();
        get_character(state);
    }

    skip_white_space(state);
    value
}

pub fn multiplication(state: &mut State) -> () {
    match_character('*', state);
    factor(state);
    emit("POP R1", state);
    emit("MULTIPLY R0, R1, R0", state);
}

pub fn name(state: &mut State) -> String {
    let mut token = String::new();

    match state.character {
        | None => expected("a name", "end of input"),
        | Some(character) if is_alphabetic(character) => {
            token += &character.to_string();
            get_character(state);
        }
        | Some(character) => expected("a name", format!("'{character}'")),
    };

    while is_alphanumeric(state.character) {
        token += &state.character.unwrap().to_string();
        get_character(state);
    }

    skip_white_space(state);
    token
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
        };
    }
}
