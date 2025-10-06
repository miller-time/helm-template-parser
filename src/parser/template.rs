use crate::{
    error::Error,
    parser::{entry::Entry, expression::parse_expression, map::Map},
    stream::TokenStream,
};

pub fn parse_template(mut input: TokenStream) -> Result<Vec<Map>, Error> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    let mut maps = Vec::new();
    let mut entries = Vec::new();
    loop {
        if input.is_empty() {
            break;
        }
        let lines = input.peek_lines(2);
        if is_expression(&lines) {
            let expression = parse_expression(&mut input)?;
            entries.push(Entry::Expression(expression));
        }
    }
    maps.push(Map {
        indentation: 0,
        items: entries,
    });
    Ok(maps)
}

fn is_expression(lines: &Vec<String>) -> bool {
    let first = lines[0].trim();
    return first.starts_with("{{") && first.ends_with("}}");
}

#[cfg(test)]
mod tests {
    use crate::{lexer::tokenize, parser::expression::Expression};

    use super::*;

    #[test]
    fn parse_valid_template() {
        let stream = tokenize("{{ foo }}").expect("failed to tokenize");
        let template = parse_template(stream).expect("failed to parse template");
        assert_eq!(
            template,
            vec![Map {
                indentation: 0,
                items: vec![Entry::Expression(Expression {
                    indentation: 0,
                    expression: " foo ".into()
                })]
            }]
        )
    }
}
