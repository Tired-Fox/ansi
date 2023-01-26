use super::tokenizer::{Tokens, Macro};
use super::types::{FormatState, Command, Color, Types};

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
    /// let ast = Lexer::convert_from("some *markup* _string_ [@Fred]with color")
    /// ```
    pub fn convert_from(text: &str) {
        let tokens = Tokens::from(text);
        Lexer::convert(tokens);
    }

    pub fn build_command(commands: Vec<(String, String)>) -> Command {
        todo!()
    }

    pub fn convert(tokens: Tokens) {
        let mut bold_state: FormatState = FormatState::Default;
        let mut uline_state: FormatState = FormatState::Default;

        let mut ast: Vec<Types> = Vec::new();

        for token in tokens.tokens() {
            match token.as_str() {
                // Match bold markup
                "*" => {
                    bold_state = match bold_state {
                        FormatState::Default | FormatState::Off => FormatState::On,
                        FormatState::On => FormatState::Off
                    };
                    ast.push(Types::Bold(bold_state))
                },
                // Match underline markup
                "_" => {
                    uline_state = match uline_state {
                        FormatState::Default | FormatState::Off => FormatState::On,
                        FormatState::On => FormatState::Off
                    };
                    ast.push(Types::Underline(uline_state))
                },
                // Match on macro `[.*]`
                _ if token.starts_with("[") && token.ends_with("]") => {
                    let commands = Macro::from(token.as_str()).tokens();

                    if commands.len() > 0 {
                        let macro_commands : Vec<Command> = Vec::new();
                        for command in commands {
                            match command.0.as_str() {
                                "@" => {
                                    match command.1.chars().nth(0) {
                                        // Color for foreground `@F.*`
                                        Some('F') => (),
                                        // Color for background `@B.*`
                                        Some('B') => (),
                                        // Color for both foreground and background `@.*`
                                        Some(_) => panic!("Color commands must start with 'F' or 'B': {}", token),
                                        // Reset all colors
                                        None => (),
                                    }
                                },
                                "~" => (),
                                "$" => (),
                                _ => (),
                            }
                        }
                        ast.push(Types::Text(token));
                    }
                    
                },
                // Match text
                _ => ast.push(Types::Text(token)),
            }
        }
        println!("{:?}", ast);
    }
}
