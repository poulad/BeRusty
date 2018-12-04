/*
clear; rustc --crate-name enums --out-dir target 3.2.enums.rs && target/enums
*/

enum WebEvent {
    PageLoaded,
    KeyPressed(char),
    Click { x: u32, y:u32 },
    PageUnloaded,
}

fn handle_event(e: WebEvent) {
    use WebEvent::*;

    match e {
        PageLoaded => {
            println!("Page loaded.")
        },
        KeyPressed(c) => println!("Key {:?} was pressed.", c),
        Click{ x, y } => {
            println!("User clicked at the point ({}, {}).", x, y)
        },
        _ => println!("Some unexpected event happened!")
    }
}

fn main() {
    use WebEvent::{PageLoaded, KeyPressed};

    handle_event(PageLoaded);
    handle_event(KeyPressed('a'));
    handle_event(WebEvent::Click{ x: 100, y: 200 });
    handle_event(WebEvent::PageUnloaded);

    // println!("WebEvent::PageLoaded has an i32 value of {}.", PageLoaded as i32);
}