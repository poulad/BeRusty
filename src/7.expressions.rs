/*
clear; rustc --crate-name expressions --out-dir target 7.expressions.rs && target/expressions
*/

fn main() {
    let foo = {
        123 + 456
    };

    let unit_type = {
        1 + 2;
    };

    println!("Value {}", foo);
    println!("{:?}", unit_type);
}