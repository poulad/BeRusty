/*
clear; rustc --crate-name match --out-dir target 8.5.match.rs && target/match
*/

fn main() {
    match_numbers();
    println!("~~~~~~~~~~~~~");

    destructure_struct();
    println!("~~~~~~~~~~~~~");

    binding();
    println!("~~~~~~~~~~~~~");

    if_let();
}

fn match_numbers() {
    for number in 1..=10 {
        if number != 1 {
            print!(", ")
        }
        match number {
            1 => print!("ONE"),
            x if x == 3 => print!("fizz"),
            4 | 5 | 6 => print!("!{}!", number),
            7...10 => print!("#{}", number),
            _ => print!("{}", number),
        }
    }
    println!();
}

fn destructure_struct() {
    struct Foo {
        bar: (i32, i32),
        baz: char,
    }

    let mut foo = Foo {
        bar: (3, 4),
        baz: 'b',
    };
    let Foo { bar: x, baz: y } = foo;

    print!("x = {:?}, y = {:?}", x, y);

    foo.baz = 'p';

    let Foo { baz: z, .. } = foo;

    println!(", z = {:?}", z)
}

fn binding() {
    fn age() -> u32 {
        return 14
    }

    match age() {
        n @ 13...19 => println!("I'm a teenager at {:?}", n),
        n => println!("I'm {:?} years old.", n)
    }
}


fn if_let() {
    let opt = Some(5);

    match opt {
        Some(x) => println!("Optional value of {:?}.", x),
        _ => {}
    }

    enum Color {
        RGB(u8,u8,u8),
        White,
    }

    let clr = Color::White;

    if let Color::RGB(r,g,b) = clr {
        println!("({:?}, {:?}, {:?})", r, g, b)
    } else if let Color::White = clr {
        println!("It's white.")
    } else {
        println!("NOT RGB!")
    }
}