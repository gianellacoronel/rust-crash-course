#![allow(unused)]

// Scalar: data types that represent a single value.
fn main(){
    //Signed integers
    // Range: -(2^(n-1)) to 2^(n-1) - 1
    // Range: -(2^(8-1)) to 2^(8-1) - 1
    let i0: i8 = -1;
    // Range: -(2^(16-1)) to 2^(16-1) - 1
    let i0: i16 = -1;
    // Range: -(2^(32-1)) to 2^(32-1) - 1
    let i0: i32 = -1;
    // Range: -(2^(64-1)) to 2^(64-1) - 1
    let i0: i64 = -1;
    // Range: -(2^(128-1)) to 2^(128-1) - 1
    let i0: i128 = -1;

    // Unsigned integers
    //Range: 0 to 2^n-1
    let u0: u8 = 0;
    // Range: 0 to 2^(16-1)
    let u0: u16 = 0;
    // Range: 0 to 2^(32-1)
    let u0: u32 = 0;
    // Range: 0 to 2^(64-1)
    let u0: u64 = 0;
    // Range: 0 to 2^(128-1)
    let u0: u128 = 0;

    // Depends on computer architecture
    let i5: isize = -6;
    let u5: usize = 6;

    //Floating point numbers
    let f0: f32 = 0.01;
    let f1: f64 = 0.02;

    // Boolean
    let b: bool = true;
    // Characters
    let c: char = 'c';
    let c: char = '🌸';

    // Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32;
    println!("{i} as u32 = {u}")

    //Min and max
    let i_max = i32::MAX;
    let u_min = u32::MIN;

    println!("i max: {i_max}");
    println!("u max: {u_max}");
}
