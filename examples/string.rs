#![allow(unused)]

// When to use String vs &str (string slice):
// - Use `String` when you need ownership or mutability.
// - Use `&str` for read-only string or string literals.
fn main() {
    // String
    let msg: String = String::from("Hello Rust 🦀");
    println!("msg: {msg}");

    let len: usize = msg.len();
    println!("String length = {len}");

    // &str
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg[..5];
    println!("slice = {s}");

    // String literals are &str
    let hello: &str = "Hello Rust";
    // Convert &str to string
    let hello = "Hello".to_string();

    // Rust automatically converts &String into a &str
    let msg: String = String::from("Hello Rust 🦀");
    print_slice(&msg);

    // Append &str to String
    let mut msg = "Hello".to_string();
    msg += " Rust";
    println!("{msg}");

    // String interpolation - format!
    let lang = "Rust";
    let emoji = "🦀";
    let msg: String = format!("Hello {} {}", lang, emoji);
    println!("{msg}");
}

fn print_slice(s: &str) {
    println!("deref: {s}");
}
