use std::{iter::Peekable, str::Chars};

enum TokenState {
    NORMAL,
    ESCAPE,
    ESCAPED,
}

impl TokenState {
    pub fn escaped(state: &TokenState) -> bool {
        match state {
            TokenState::ESCAPE | TokenState::ESCAPED => true,
            _ => false,
        }
    }

    pub fn single_escape(state: &TokenState) -> bool {
        match state {
            TokenState::ESCAPE => true,
            _ => false,
        }
    }
}

pub struct Macro {
    tokens: Vec<(String, String)>,
}

impl Macro {
    pub fn tokens(&self) -> Vec<(String, String)> {
        self.tokens.clone()
    }

    pub fn new() -> Self {
        Macro { tokens: Vec::new() }
    }

    pub fn from(data: &str) -> Self {
        let mut tokens = Macro::new();
        tokens.feed(data);
        tokens
    }

    pub fn feed(&mut self, data: &str) {
        let mut text: Chars = data[1..data.len() - 1].chars();

        let mut command = (String::new(), String::new());

        let mut new_command = |s: &str, c: &mut (String, String)| {
            if !c.0.is_empty() {
                self.tokens.push(c.clone());
            }
            *c = (s.to_string(), String::new())
        };

        while let Some(c) = text.next() {
            match c {
                '@'  => new_command("@", &mut command),
                '$'  => new_command("$", &mut command),
                '~'  => new_command("~", &mut command),
                _ => match command.0.as_str() {
                    "$" => panic!("Escape command takes no additional data: {}", data),
                    _ => command.1.push(c),
                },
            };
        }

        if !command.0.is_empty() {
            self.tokens.push(command)
        }
    }
}

pub struct Tokens {
    tokens: Vec<String>,
}

impl Tokens {
    pub fn tokens(&self) -> Vec<String> {
        self.tokens.clone()
    }

    pub fn new() -> Self {
        Tokens { tokens: Vec::new() }
    }

    pub fn from(data: &str) -> Self {
        let mut tokens = Tokens::new();
        tokens.feed(data);
        tokens
    }

    fn consume_macro(&mut self, data: &mut Peekable<Chars>, state: &mut TokenState) {
        let mut command = String::from("[");

        match data.peek() {
            Some(c) if !vec!['@', '~', '$'].contains(c) => {
                panic!("Macro's must start with a command. Commands start with @, ~, or $")
            }
            None | Some(_) => (),
        }

        while let Some(m) = data.next() {
            match m {
                ']' if !TokenState::single_escape(state) => {
                    command.push(m);
                    self.tokens.push(command.clone());
                    return;
                }
                '\\' if !TokenState::escaped(state) => {
                    *state = TokenState::ESCAPE;
                }
                _ => {
                    command.push(m);
                    match state {
                        TokenState::ESCAPE => *state = TokenState::NORMAL,
                        _ => (),
                    }
                }
            }
        }

        panic!("Macro was not closed")
    }

    pub fn feed(&mut self, data: &str) {
        let mut data: Peekable<Chars> = data.chars().peekable();

        let mut token: String = String::new();
        let mut state: TokenState = TokenState::NORMAL;

        while let Some(c) = data.next() {
            match c {
                '[' if !TokenState::single_escape(&state) => {
                    if !token.is_empty() {
                        self.tokens.push(token.clone());
                        token = String::new();
                    }

                    self.consume_macro(&mut data, &mut state);
                }
                '*' | '_' if !TokenState::escaped(&state) => {
                    if !token.is_empty() {
                        self.tokens.push(token.clone());
                        token = String::new();
                    }
                    self.tokens.push(String::from(c))
                }
                '\\' if !TokenState::escaped(&state) => {
                    state = TokenState::ESCAPE;
                }
                _ => {
                    token.push(c);
                    state = match state {
                        TokenState::ESCAPE => TokenState::NORMAL,
                        _ => state,
                    }
                }
            };
        }

        if !token.is_empty() {
            self.tokens.push(token)
        }
    }
}
