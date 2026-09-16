use crate::state::State;
use std::{
    io::{Write, stdout},
    process::exit,
};

pub fn abort(message: impl Into<String>) -> ! {
    error(message);
    exit(1);
}

pub fn emit(instruction: impl Into<String>, state: &State) -> () {
    let mut writer: Box<dyn Write> = match &state.output_file {
        | None => Box::new(stdout()),
        | Some(file) => Box::new(file),
    };

    if let Err(error) = writer.write((instruction.into() + "\n").as_bytes()) {
        abort(format!("Failed to emit instruction: {error}"));
    }
}

pub fn error(message: impl Into<String>) -> () {
    eprintln!("Error: {}", message.into());
}

pub fn expected(expected: impl Into<String>, found: impl Into<String>) -> ! {
    abort(format!("Expected {}, found {}.", expected.into(), found.into()));
}
