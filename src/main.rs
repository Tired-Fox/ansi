use ansi::markup::Parser;
use ansi::{saiml, print_saiml};

fn main() {
    // Hel[@Fred$]lo W*or_ld[@$]*!
    println!("{}", saiml!("Hel[@Fred@Bwhite$~https://example.com]lo W*or_ld[@$]* Bold Text!"));
    print_saiml!("[@Fblue]*_Hello World!")
}
