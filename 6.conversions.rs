/*
clear; rustc --crate-name conversions --out-dir target 6.conversions.rs && target/conversions
*/

use std::convert::*;
use std::string::ToString;

#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number{ value: item }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        format!("~{}~", self.value)
    }
}

fn main() {
    let mut n = Number::from(123);
    println!("{:?}", n);

    n = 456.into();
    println!("{:?}", n);

    println!("Stringified: {}", n.to_string());

    let result = "789".parse::<u16>();
    println!("{:?}", result);
    let parsed_number = result.unwrap();
    println!("{:?}", parsed_number);
}