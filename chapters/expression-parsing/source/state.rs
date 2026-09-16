use std::fs::File;

pub struct State {
    pub character: Option<char>,
    pub input_file: Option<File>,
    pub output_file: Option<File>,
}
