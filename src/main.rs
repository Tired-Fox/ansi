mod markup;
use markup::Parser;
use markup::types::{Types, FormatState, Command, Color};

mod style;
use style::{Color as Style_Color, Style};

fn main() {
    // Hel[@Fred$]lo W*or_ld[@$]*!

    let sample_ast = vec![
        Types::Text("Hel".to_string()),
        Types::Commands(vec![Command::Color(Color::Foreground("red".to_string()))]),
        Types::Text("lo W".to_string()),
        Types::Bold(FormatState::On),
        Types::Text("or".to_string()),
        Types::Underline(FormatState::On),
        Types::Text("ld".to_string()),
        Types::Commands(vec![Command::Color(Color::Reset)]),
        Types::Bold(FormatState::Off),
        Types::Text("!".to_string()),
    ];
    println!("{:?}", sample_ast);

    let result = Parser::from("Hel[@Fred@Bwhite$~https://example.com]lo W*or_ld[@$]* Bold Text!");
    println!("{}", result);

    let style = Style::new()
        .bold()
        .foreground(Style_Color::hex("#D60620").unwrap());
    println!("{}", style.format("Hello World!"))
}
