/*
clear; rustc --crate-name methods --out-dir target 9.1.methods.rs && target/methods
*/

fn main() {
    println!("Static method call: {:?}", Point::origin());
    println!("Ctor: {:?}", Point::new(3.0, 4.0));

    let mut rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(2.0, 4.0),
    };
    println!("Rectangle's area: {:?}", rect.area());

    rect.double();
    println!("Rectangle is doubled. area: {:?}", Rectangle::area(&rect));
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    fn origin() -> Point {
        Point::new(0.0, 0.0)
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let width = (self.p1.x - self.p2.x).abs();
        let height = (self.p1.y - self.p2.y).abs();
        width * height
    }

    fn double(&mut self) {
        self.p1.x *= 2.0;
        self.p1.y *= 2.0;
        self.p2.x *= 2.0;
        self.p2.y *= 2.0;
    }
}
