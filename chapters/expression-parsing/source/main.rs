use crate::{input::get_character, output::abort, parse::expression, state::State};
use std::fs::File;

mod input;
mod output;
mod parse;
mod state;

const INPUT_FILE_PREFIX: &str = "--input-file=";
const OUTPUT_FILE_PREFIX: &str = "--output-file=";

fn main() -> () {
    let mut arguments = std::env::args().rev();
    let input_file_path = arguments.find_map(|argument| argument.strip_prefix(INPUT_FILE_PREFIX).map(str::to_string));
    let mut arguments = std::env::args().rev();
    let output_file_path = arguments.find_map(|argument| argument.strip_prefix(OUTPUT_FILE_PREFIX).map(str::to_string));

    let input_file = match input_file_path {
        | None => None,
        | Some(path) => match File::open(&path) {
            | Err(error) => abort(format!("Failed to open file '{path}': {error}")),
            | Ok(value) => Some(value),
        },
    };
    let output_file = match output_file_path {
        | None => None,
        | Some(path) => match File::create(&path) {
            | Err(error) => abort(format!("Failed to create file '{path}': {error}")),
            | Ok(value) => Some(value),
        },
    };

    let mut state = State { character: None, input_file, output_file };
    get_character(&mut state);

    expression(&mut state);
}
