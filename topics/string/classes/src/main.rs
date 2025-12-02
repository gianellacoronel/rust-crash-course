#![allow(unused)]

//String and &str (string slice)
// - Use `String` when you need ownership or mutability.
// - Use `&str` for read-only string or string literals.
fn main() {
    let msg: String = String::from("Hello Rust");
    let msg: String = "Hello Rust".to_string();

    let length: usize = msg.len(); //unsize type adjust to the number of bits that represent this inside integer, which is 64 bits for a 64-bit architecture.

    let s: &str = &msg[0..5]; //0 to the 5th character
    println!("s = {}", s); //s = Hello

    let s: &str = "Hello World";
    let x: String = s.to_string();

    //Rust automatically converts &String into &str
    let msg: String = String::from("Hello Rust");
    print(&msg);

    let s: &str = "Hello World";
    print(s);

    // Append &str to String
    let mut msg: String = String::from("Hello Rust");
    msg += " World";
    println!("{msg}"); //Hello Rust World

    // String interpolation - format!
    let lang = "Rust";
    let emoji = "🦀"
    let s = "Hello Rust 🦀" // We want this

    let mut s = "Hello".to_string();
    s += " ";
    s += lang;
    s += " ";
    s += emoji;

    // Using format! macro
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");
}

//fn print(s: &String) { //&String: is reference to a String
fn print(s: &str){ //we have to put it with this type to match with function call ( print(&msg) )
    println!("{s}");
}
