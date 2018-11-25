/*
clear; rustc --crate-name types --out-dir bin 5.types.rs && bin/types
*/

type CharacterCode = u8;

fn print_character(c: CharacterCode) {
    println!("{:?}", c as char)
}

fn main() {
    print_character(65)
}