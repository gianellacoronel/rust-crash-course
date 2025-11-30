#![allow(unused)]

// Array - collection of elements with length known at compile time
// Slice - collection of elements with length not known at compile time

fn main() {
    // Array
    //      type; size
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[0]: {}", arr[0]);

    //Write
    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 99;

    // [start, end] => Rust know that have to generate that array with 10 elements with value 0.
    let arr: [u32; 10] = [0; 10];
    println!("arr: {:?}", arr);

    // Slice
    let nums: [i32; 10] = [-1, 1, -2, 2, -3, 3, -4, 4, -5, 5];
     // First 3 elements (indexes = 0, 1, 2)
     let s: &[i32] = &nums[0..3]; //or &nums[..3]  //This involves index 0, 1, 2
     // Last 3 elements (indexes = 7, 8, 9)
     let s: &[i32] = &nums[7..10]; //or &nums[7..] //This involves index 7, 8, 9
     // Middle 4 elements (indexes = 3, 4, 5, 6)
     let s: &[i32] = &nums[3..7];
     println!("s: {:?}", s);
}
