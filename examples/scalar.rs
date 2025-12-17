#![allow(unused)]

// Scalar - data types that represent a single value
fn main() {
    // Signed Integes and Unsigned Integers (Знаковые и беззнаковые целые числа)
    // Boolean
    // Character


    let a: u8 = 255; // Unsigned 8-bit integer
    let b: i8 = -128; // Signed 8-bit integer

    let c: u16 = 65535; // Unsigned 16-bit integer
    let d: i16 = -32768; // Signed 16-bit integer

    let e: u32 = 4294967295; // Unsigned 32-bit integer
    let f: i32 = -2147483648; // Signed 32-bit integer

    let g: u64 = 18446744073709551615; // Unsigned 64-bit integer
    let h: i64 = -9223372036854775808; // Signed 64-bit integer

    let i: u128 = 340282366920938463463374607431768211455; // Unsigned 128-bit integer
    let j: i128 = -170141183460469231731687303715884105728; // Signed 128-bit integer

    // Depends on compiler architecture (Целочисленные типы зависят от архитектуры компилятора)
    let i5: isize = -6; // Pointer-sized signed integer
    let u5: usize = 6;  // Pointer-sized unsigned integer

    // Floating-Point Types
    let m: f32 = 3.14; // 32-bit floating point
    let n: f64 = 2.718281828459045; // 64-bit floating point

    // Boolean Type
    let o: bool = true;
    let p: bool = false;

    // Character Type
    let q: char = 'R';
    let r: char = '😊';

    // Type conversion
    let i: i32 = -1;
    let u: u32 = i as u32; // Convert signed to unsigned
    println!("{i} as u32 is {u}");

    // Min and Max values
    let i_max = i32::MAX;
    let u_min = u32::MIN; 
    println!("i max: {i_max}");
    println!("u min: {u_min}");
}     