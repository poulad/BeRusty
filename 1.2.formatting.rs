/*
rustc --crate-name formatting 1.2.formatting.rs && ./formatting
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
