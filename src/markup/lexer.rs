use super::tokenizer::{Macro, Tokens};
use super::types::{Color, Command, FormatState, Types};

pub struct Lexer;

impl Lexer {
    /// Convert a markup string into an AST
    ///
    /// Start by tokenizing the string with the tokenizer then converting each token into it's
    /// appropriate type.
    ///
    /// # Example
    /// ```
    /// use ansi::lexer::Lexer;
    ///
    /// let ast = Lexer::from("some *markup* _string_ [@Fred]with color")
    /// ```
    pub fn from(text: &str) -> Vec<Types> {
        let tokens = Tokens::from(text);
        Lexer::convert(tokens)
    }

    pub fn convert(tokens: Tokens) -> Vec<Types> {
        let mut bold_state: FormatState = FormatState::Default;
        let mut uline_state: FormatState = FormatState::Default;

        let mut ast: Vec<Types> = Vec::new();

        let mut escaped = false;
        for token in tokens.tokens() {
            match token.as_str() {
                // Match bold markup
                "*" if !escaped => {
                    bold_state = match bold_state {
                        FormatState::Default | FormatState::Off => FormatState::On,
                        FormatState::On => FormatState::Off,
                    };
                    ast.push(Types::Bold(bold_state))
                }
                // Match underline markup
                "_" if !escaped => {
                    uline_state = match uline_state {
                        FormatState::Default | FormatState::Off => FormatState::On,
                        FormatState::On => FormatState::Off,
                    };
                    ast.push(Types::Underline(uline_state))
                }
                // Match on macro `[.*]`
                _ if token.starts_with("[") && token.ends_with("]") => {
                    let commands = Macro::from(token.as_str()).tokens();

                    if commands.len() > 0 {
                        let mut macro_commands: Vec<Command> = Vec::new();
                        for command in commands {
                            match command.0.as_str() {
                                "@" => {
                                    match command.1.chars().nth(0) {
                                        // Color for foreground `@F.*`
                                        Some('F') => macro_commands.push(Command::Color(
                                            Color::Foreground(command.1[1..].to_string()),
                                        )),
                                        // Color for background `@B.*`
                                        Some('B') => macro_commands.push(Command::Color(
                                            Color::Background(command.1[1..].to_string()),
                                        )),
                                        // Color for both foreground and background
                                        Some(_) => {
                                            macro_commands.push(Command::Color(Color::Foreground(
                                                command.1.clone(),
                                            )));
                                            macro_commands
                                                .push(Command::Color(Color::Background(command.1)));
                                        }
                                        // Reset all colors
                                        None => macro_commands.push(Command::Color(Color::Reset)),
                                    }
                                }
                                "~" => macro_commands.push(Command::Url(command.1)),
                                "$" => {
                                    escaped = !escaped;
                                }
                                _ => (),
                            }
                        }
                        ast.push(Types::Commands(macro_commands));
                    }
                }
                // Match text
                _ => ast.push(Types::Text(token)),
            }
        }
        ast
    }
}
