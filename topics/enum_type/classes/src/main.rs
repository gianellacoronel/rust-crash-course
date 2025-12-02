#![allow(unused)]

//(1.1) We have to put special macro to make this code works!
#[derive(Debug, PartialEq)] //This also works with structs, and with enums   //(2.1) Add PartialEq
//PartialEq allows us to compare values of enums
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}
fn main(){
    let cmd: Command = Command::Play;
    let cmd: Command = Command::Skip(10);
    let cmd: Command = Command::Resize { width: 100, height: 50 };

    println!("{}", cmd); //(1.0) This gives an error: "Command cannot be formatted with the default formatter"

    //Debug
    println!("{:?}", cmd); //(1.3) This prints the code

    let cmd0: Command = Command::Play;
    let cmd1: Command = Command::Skip(10);
    println!("{}", cmd0 == cmd1); //(2.0) This gives an error: "enum Command must implement 'PartialEq'"

    // Option
    // Option<T> = Some(T) | None
    // Express a presence of a value or the absence of the value
    let x: Option<i32> = Some(1);
    let x: Option<i32> = None; // There is no value

    // Result
    // Result<T, E> = Ok(T) | Error(E)
    // Represent something successfully executing, returning a value of the type T, or there was an error and it returns the error of type E
    // "100" -> 100
    let x: Result<i32, String> = Ok(100);
    // "123vxvf" -> error
    let x: Result<i32, String> = Err("Failed to parse string into a number".to_string());
}
