use crate::{
    Files,
    input::{get_character, skip_white_space},
};
use std::fs::File;

pub struct State {
    pub character: Option<char>,
    pub input_file: Option<File>,
    pub output_file: Option<File>,
}

impl State {
    pub fn initialize(files: Files) -> Self {
        let mut state = Self {
            character: None,
            input_file: files.input,
            output_file: files.output,
        };

        get_character(&mut state);
        skip_white_space(&mut state);

        state
    }
}
