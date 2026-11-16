#![allow(unused)]

use crate::{output::abort, state::State};
use std::fs::File;

mod input;
mod output;
mod parse;
mod state;

const INPUT_FILE_PREFIX: &str = "--input-file=";
const OUTPUT_FILE_PREFIX: &str = "--output-file=";

struct Arguments {
    input_file_path: Option<String>,
    output_file_path: Option<String>,
}

impl Arguments {
    fn parse() -> Self {
        let mut arguments = std::env::args().rev();
        let input_file_path = arguments.find_map(|argument| argument.strip_prefix(INPUT_FILE_PREFIX).map(str::to_string));
        let mut arguments = std::env::args().rev();
        let output_file_path = arguments.find_map(|argument| argument.strip_prefix(OUTPUT_FILE_PREFIX).map(str::to_string));

        Self {
            input_file_path,
            output_file_path,
        }
    }
}

struct Files {
    input: Option<File>,
    output: Option<File>,
}

impl Files {
    fn open(arguments: Arguments) -> Self {
        let input_file = match arguments.input_file_path {
            | None => None,
            | Some(path) => match File::open(&path) {
                | Err(error) => abort(format!("Failed to open file '{path}': {error}")),
                | Ok(value) => Some(value),
            },
        };
        let output_file = match arguments.output_file_path {
            | None => None,
            | Some(path) => match File::create(&path) {
                | Err(error) => abort(format!("Failed to create file '{path}': {error}")),
                | Ok(value) => Some(value),
            },
        };

        Self {
            input: input_file,
            output: output_file,
        }
    }
}

fn main() -> () {
    let arguments = Arguments::parse();
    let files = Files::open(arguments);
    let mut state = State::initialize(files);
}
