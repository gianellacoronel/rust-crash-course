pub fn hello() -> String {
    String::from("Hello Rust")
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    s += "!";
    return s;
}
