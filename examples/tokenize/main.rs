use std::fs;

use helm_template_parser::lexer::tokenize;

fn main() {
    let contents = fs::read_to_string("samples/clusterrole.yaml")
        .expect("failed to open sample clusterrole.yaml");
    let tokens = tokenize(&contents).expect("failed to tokenize");
    dbg!(tokens);
}
