
#[derive(Debug, Clone)]
pub enum Color {
    Foreground(String),
    Background(String),
    Reset
}

#[derive(Debug, Clone)]
pub enum Command {
    Url(String),
    Color(Color),
    Escaped,
}

#[derive(Debug, Clone, Copy)]
pub enum FormatState {
    On,
    Off,
    Default
}

#[derive(Debug, Clone)]
pub enum Types {
    Text(String),
    Bold(FormatState),
    Underline(FormatState),
    Commands(Vec<Command>)
}