mod my_funcs;
// import nested folder
mod other_funcs;

// use crate::my_funcs::*; // use all functions from my_funcs module
// use crate::my_funcs::subract_five;
use crate::my_funcs::{add_five, subract_five};
use crate::other_funcs::multiply::multiply;

// rust is statically typed language
// rust is a compiled language => cargo run

// fn main() => main entry point for rust programs
fn main() {
    // Every variable in rust is immutable by default
    // unsigned 32bit integer (positive whole number)
    let x: u32 = 5;
    print!("immutable variable X: {}\n", x);

    // use mut keyword to make variable mutable
    // signed 32bit integer (positive/negative whole number)
    let mut y: i32 = 6;
    y = add_five(y);
    y = subract_five(y);
    y = multiply(y, y);
    print!("mutable variable Y: {}\n", y);
}

// use underscore to ignore warning unused variables or functions
fn _my_function() {
    let _x = 5;
}