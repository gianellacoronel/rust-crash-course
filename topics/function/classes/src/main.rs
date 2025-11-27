#![allow(unused)]

//To return more than one value
// fn add(x: u32, y: u32) -> (u32, bool){
//     return (x + y, true);
// }

fn add_with_return(x: u32, y: u32) -> u32{
    return x + y;
}

fn add(x: u32, y: u32) -> u32{
    x + y //here we don't use "return" and also remove the semicolon, but it still returns that value
}

//example of a function that do not return nothing (without ->)
fn print(s: String){
    println!("{s}{s}{s}{s}{s}");
}

fn main(){
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");

    print("🌸".to_string());
}
