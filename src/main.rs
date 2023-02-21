use ansi::markup::Parser;
use ansi::{saiml, print_saiml};

fn main() {
    // Hel[@Fred$]lo W*or_ld[@$]*!

    let result = Parser::from("Hel[@Fred@Bwhite$~https://example.com]lo W*or_ld[@$]* Bold Text!");
    println!("{}", result);

    print_saiml!("[@Fblue]*_Hello World!")
}
