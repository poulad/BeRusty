/*
clear; rustc --crate-name formatting_display --out-dir target 1.2.2.display.rs && target/formatting_display
*/

use std::fmt;

#[derive(Debug)]
struct Point2D(i32, i32);

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {:b} , {:b} }}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imaginary: f32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:+}i", self.real, self.imaginary)
    }
}

struct NumbersList(Vec<i32>);
impl fmt::Display for NumbersList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")? }
            write!(f, "{}: {}", count, v)?
        }
        write!(f, "]")
    }
}

fn main() {
    println!("{}", Point2D(1, 2));
    println!("{:?}", Point2D(3, 4));
    println!("{:#?}", Point2D(5, 6));
    println!("{:b}", Point2D(7, 8));

    println!("{}", Complex{real: 3.3, imaginary: 7.2});
    println!("{:?}", Complex{real: 3.3, imaginary: 7.2});

    let v = NumbersList(vec![1,2,3,4,5]);
    println!("{}", v)
}