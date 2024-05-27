use chumsky::{extra::ParserExtra, prelude::*, text::whitespace};

use crate::CRLF;

#[derive(Debug)]
enum Token<'a> {
    Helo,
    Text(&'a str),
}

#[derive(Debug)]
pub(crate) enum Command<'a> {
    Helo(&'a str),
    Quit,
}
// fn unicode_sequence<'src>() -> impl Parser<'src, &'src str, TokenKind, LexerExtra>

pub(crate) type ParseExtra<'a> = extra::Err<Simple<'a, char>>;

fn hostname<'src>() -> impl Parser<'src, &'src str, &'src str, ParseExtra<'src>> {
    any()
        .filter(|c: &char| c == &'.' || c.is_alphanumeric())
        .repeated()
        .to_slice()
}

fn helo<'src>() -> impl Parser<'src, &'src str, Command<'src>, ParseExtra<'src>> {
    just("HELO")
        .then_ignore(whitespace().at_least(1))
        .then(hostname())
        .then_ignore(just(CRLF))
        .map(|(_, text): (&str, &str)| {
            let text = if let Some(index) = text.find(CRLF) {
                if index > 0 {
                    &text[0..index]
                } else {
                    ""
                }
            } else {
                text
            };

            Command::Helo(text)
        })
}

fn quit<'src>() -> impl Parser<'src, &'src str, Command<'src>, ParseExtra<'src>> {
    just("QUIT")
        .ignored()
        .then_ignore(just(CRLF))
        .map(|_| Command::Quit)
}

pub(crate) fn parser<'src>() -> impl Parser<'src, &'src str, Command<'src>, ParseExtra<'src>> {
    choice((helo(), quit())).then_ignore(just("\0").repeated())
}

#[cfg(test)]
mod tests {
    use super::parser;
    use chumsky::prelude::*;

    #[test]
    fn helo() {
        let input = "HELO relay.example.org";
        let (output, errors) = parser().parse(input).into_output_errors();
        println!("{output:#?}")
    }
}
