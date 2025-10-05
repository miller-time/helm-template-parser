use crate::{
    error::Error, parser::expression::parse_expression, stream::TokenStream, token::Token,
};

#[derive(Debug, PartialEq)]
pub enum TemplateEntry {
    Expression(String),
    Map,
}

#[derive(Debug, Default, PartialEq)]
pub struct Template {
    pub entries: Vec<TemplateEntry>,
}

pub fn parse_template(mut input: TokenStream) -> Result<Template, Error> {
    if input.is_empty() {
        return Ok(Template::default());
    }
    let mut entries = Vec::new();
    loop {
        match input.peek() {
            Some(token) => match token {
                Token::LeftBrace => {
                    let expression = parse_expression(&mut input)?;
                    entries.push(TemplateEntry::Expression(expression));
                }
                _ => {}
            },
            None => break,
        }
    }
    Ok(Template { entries })
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn parse_valid_template() {
        let stream = tokenize("{{ foo }}").expect("failed to tokenize");
        let template = parse_template(stream).expect("failed to parse template");
        assert_eq!(
            template,
            Template {
                entries: vec![TemplateEntry::Expression(" foo ".into())]
            }
        )
    }
}
