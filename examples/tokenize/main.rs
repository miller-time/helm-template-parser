use std::fs;

use helm_template_parser::lexer::tokenize;

fn main() {
    let samples = fs::read_dir("samples").expect("failed to read samples directory");
    for entry in samples {
        let entry = entry.expect("failed to read sample");
        let file_name = entry.file_name();
        let file_name = file_name.to_str().expect("failed to read sample file name");
        if !file_name.ends_with(".yaml") {
            continue;
        }
        let contents = fs::read_to_string(entry.path())
            .unwrap_or_else(|_| panic!("failed to open sample {}", file_name));
        let tokens = tokenize(&contents)
            .unwrap_or_else(|_| panic!("failed to tokenize sample {}", file_name));
        dbg!(file_name, tokens.len());
    }
}
