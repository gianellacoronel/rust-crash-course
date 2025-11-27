#![allow(unused)]
//Constants could be inside the main function

fn main() {
    //Variables
    // - Immutable by default
    // - Use mut keyword to make it mutable
    let x = 1;

    //We have to put 'mut'
    //w += 1;  it's not permited
    let mut w = 2;

    // Type inference, by default type i32
    let y: i32 = -1;
    let z = -1;

    //Shadowing: we can redeclare the value, with different types and values
    let x: i32 = 1;
    let x: i32 = 2;
    let x: bool = true;

    //Type placeholder
    //We can use _ to let Rust to figure out the type
    let x: _ = 1235;

    //Constants
    const NUM: u32 = 3408;

    //println!
    let x = 1;
    println!("x is {}", x);
    //Inline
    println!("x is {x}");

    let z = x + x;
    //Positional arguments
    println!("{x} + {x} = {z}");
    // index 0 : x
    // index 1 : x + x
    println!("{0} + {0} = {1}", x, x + x);

}
