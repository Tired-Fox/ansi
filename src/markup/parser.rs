use super::lexer::Lexer;
use super::types::{Color, Command, FormatState, Types};

pub struct Parser;

impl Parser {
    pub fn from(text: &str) -> String {
        let ast = Lexer::from(text);
        Parser::parse(ast)
    }

    pub fn parse(ast: Vec<Types>) -> String {
        let mut result = String::new();
        let mut style: Vec<&'static str> = Vec::new();

        let mut push_style = |s: &mut Vec<&str>, r: &mut String| {
            
        };

        for node in ast {
            match node {
                Types::Text(data) => {
                    if style.len() > 0 {
                        let mut markup = String::from("\x1b[");
                        markup.push_str(style.join(";").as_str());
                        markup.push('m');
        
                        result.push_str(markup.as_str());
                        style = Vec::new()
                    }
                    result.push_str(data.as_str())
                },
                Types::Bold(state) => match state {
                    FormatState::On => style.push("1"),
                    FormatState::Off => style.push("22"),
                    _ => ()
                },
                Types::Underline(state) => match state {
                    FormatState::On => style.push("4"),
                    FormatState::Off => style.push("24"),
                    _ => ()
                },
                Types::Commands(commands) => {
                    // Color and url commands
                    ()
                },
            }
        }
        String::from(result + "\x1b[0m")
    }
}
