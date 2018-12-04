/*
clear; rustc --crate-name flow_control --out-dir target 8.flow-control.rs && target/flow_control
*/

fn main() {
    println!("~~~~~~~~~~~~~");

    labled_loops();
    println!("~~~~~~~~~~~~~");

    returning_loop();
    println!("~~~~~~~~~~~~~");

    while_fizzbuzz(30);
    println!("~~~~~~~~~~~~~");

    for_fizzbuzz(20);
    println!("~~~~~~~~~~~~~");
}

fn labled_loops() {
    'outer: loop {
        println!("Beginning of the outer loop.");

        'inner: loop {
            println!("Beginning of the inner loop.");

            'innerer: loop {
                println!("Beginning of the innerer loop.");
                break 'outer;

                println!("End of the innerer loop.");
            }

            println!("End of the inner loop.");
        }
        println!("End of the outer loop.");
    }

    println!("FINITO!");
}

fn returning_loop() {
    let mut counter = 0;
    let a = loop {
        if counter == 20 { break counter }
        counter += 1
    };
    println!("a = {}", a);
    assert_eq!(a, 20);
}

fn while_fizzbuzz(max: u32) {
    let mut n = 1;
    while n < max + 1 {
        if n > 1 { print!(", ") }

        if n % 15 == 0 {
            print!("fizzbuzz")
        } else if n % 3 == 0 {
            print!("fizz")
        } else if n % 5 == 0 {
            print!("buzz")
        } else {
            print!("{}", n)
        }

        n += 1
    }

    println!("")
}

fn for_fizzbuzz(n: u32) {
    for i in 1..=n {
        if i != 1 { print!(", ") }
        if i % 15 == 0 {
            print!("fuzzbuzz")
        } else if i % 3 == 0 {
            print!("fizz")
        }
        else if i % 5 == 0 {
            print!("buzz")
        } else {
            print!("{}", i)
        }
    }
    println!()
}