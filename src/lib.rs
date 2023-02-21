#![doc = include_str!("../README.md")]
pub mod style;
pub mod markup;

use markup::Parser;

#[macro_export]
macro_rules! saiml {
    ($text:literal) => {
        Parser::from($text)   
    };
    () => { String::new() }
}

#[macro_export]
macro_rules! print_saiml {
    ($text:literal) => {
        println!("{}", Parser::from($text))  
    };
    () => { println!() }
}