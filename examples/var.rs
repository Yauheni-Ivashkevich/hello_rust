#![allow(unused)]

 // Constants 
    const NUM: u32 = 1;

fn main() {
    // Variables in Rust
    // - Immutable by default
    // - Use mut keyword to make it mutable
    // let's declare some variables
    let mut x = 1;
    x += 1;

    // Types inference
    let y: i32 = -1;
    let z = -1;

    // Shadowing
    let x: i32 = 1;
    let x: i32 =2 ;
    let x: bool = true;

    // Type placeholder
    let x:_ = 1234;

    // println!
    let x = 1;
    println!("x is {}", x);
    // InLine
    println!("x is {x}");
    // Positional Arguments
    println!("{0} + {0} = {1}", x, x + x);
    // Debug 
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: x {:#?}", x);
}