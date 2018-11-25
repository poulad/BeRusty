/*
clear; rustc --crate-name bindings 4.variable-bindings.rs && ./bindings
*/

fn main() {
    let _immutable = 23;
    let mut mutable = _immutable;

    mutable += 23;
    println!("{}", mutable)
}