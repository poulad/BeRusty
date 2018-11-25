/*
clear; rustc --crate-name types --out-dir target 5.types.rs && target/types
*/

type CharacterCode = u8;

fn print_character(c: CharacterCode) {
    println!("{:?}", c as char)
}

fn main() {
    print_character(65)
}