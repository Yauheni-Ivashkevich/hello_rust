#![allow(unused)]

use std::u32::MAX;

// Overflow doesn't panic when compiling with --release
fn main() {
    let mut x = u32::MAX;
    x += 1;
    println!("u32 max: {}, x: {}", u32::MAX, x);

    // u32:: checked_add - returns None on overflow
    let x = u32 :: checked_add(3, 1);
    println!("checked_add: {:?}", x);

    // u32:: wrapping_add - explicitly allows overflow (wraps around) 
    let x = u32 :: wrapping_add(u32 :: MAX, 1);
    println!("wrapping_add: {:?}", x);
}