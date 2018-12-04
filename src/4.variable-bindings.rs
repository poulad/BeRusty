/*
clear; rustc --crate-name bindings --out-dir target 4.variable-bindings.rs && target/bindings
*/

fn main() {
    let _immutable = 23;
    let mut mutable = _immutable;

    mutable += 23;
    println!("{}", mutable)
}