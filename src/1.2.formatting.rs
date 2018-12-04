/*
clear; rustc --crate-name formatting --out-dir target 1.2.formatting.rs && target/formatting
*/

#[derive(Debug)]
struct PrintableFoo{ }

#[derive(Debug)]
struct Dude {
    age: i32,
    name: &'static str
}

fn main() {
    println!("1.2 Formatting");

    println!("{:?}", PrintableFoo{});

    println!("{:#?}", Dude{age: 25, name: "Poulad"})
}
