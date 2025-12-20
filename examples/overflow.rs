#![allow(unused)]

// Overflow doesn't panic when compiling with --release
fn main() {
    let mut x = u32::MAX;
    x += 1;
    println!("u32 max: {}, x: {}", u32::MAX, x);

    // u32:: checked_add - returns None on overflow
    // u32:: wrapping_add - explicitly allows overflow (wraps around) 

}