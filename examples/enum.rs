#![allow(unused)]

#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Skip(u32),
    Back(u32),
    Resize { width: u32, height: u32 },
}

fn main() {
    let cmd = Command::Play;
    let cmd = Command::Skip(60);
    let cmd = Command::Resize {
        width: 500,
        height: 300,
    };

    // Option<T> = Some(T) | None
    let x: Option<i32> = Some(-1);
    let x: Option<i32> = None;
    println!("{:?}", x);

    // Result<T, E> = Ok(T) | Error(E)
    let res: Result<i32, &str> = Ok(100);
    let res: Result<i32, &str> = Err("error 💀");
    println!("{:?}", res);
}