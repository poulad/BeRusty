/*
clear; rustc --crate-name primitives 2.primitives.rs && ./primitives
*/

fn switch(pair: (i32, bool)) -> (bool, i32) {
    (pair.1, pair.0)
}

use std::mem;
fn analyze_slice(slice: &[i32]) {
    println!("Slice {slice:?} has {length} elements and the size of {size}.",
        slice=slice,
        size=mem::size_of_val(slice),
        length=slice.len()
    )
}

fn main() {
    println!("{:?}", switch((2, true)));

    let array = [-1, 2, 5, -967, 45612, -45, 6];

    analyze_slice(&array[2..5])
}