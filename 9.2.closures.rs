/*
clear; rustc --crate-name closures --out-dir target 9.2.closures.rs && target/closures
*/

fn main() {
    closure_capturing();
    println!("~~~~~~~~~~~~~");

    input_closures();
}

fn closure_capturing() {
    let mut i = 0;
    {
        let mut mutate = || {
            i += 1;
            println!("i = {:?}", i)
        };

        mutate();
        mutate();

        // let _j = i;
    }

    let mut j = i;
    j += 1;
    println!("borrowed. i = {:?}, j = {:?}", i, j)
}

fn input_closures() {
    fn apply<F>(func: F)
    where
        F: FnOnce(),
    {
        func();
    }

    let n = 5;
    let fn_double = || {
        println!("{:?}", n * 2);
    };
    apply(fn_double);
}

fn diverging_func() -> ! {
    panic!("Oh, no!")
}
