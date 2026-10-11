#![allow(unused_variables)]

fn main() {
    let x = 5;
    let x = x * 2;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }

    println!("The value of x is {}", x);

    // Data Types
    let guess: u32 = "42".parse().expect("Not a number");

    /*
    | Length                 | Signed | Unsigned |
    |------------------------|--------|----------|
    | 8 bits                 | i8     | u8       |
    | 16 bits                | i16    | u16      |
    | 32 bits                | i32    | u32      |
    | 64 bits                | i64    | u64      |
    | 128 bits               | i128   | u128     |
    | Architecture-dependent | isize  | usize    |
    */

    let byte = b'a';

    println!("{:b}", byte); // 0110_0001

    // TODO: learn about wrapping_* methods later

    let money: f32 = 15.2;
    let money: f64 = 0.03;

    // operations
    let sum = 5 + 10; // addition
    let sub = 5 - 10; // subtraction
    let multi = 5 * 10; // multiplication
    let div = -5 / 10; // division
    let remain = 5 % 10; // remainder

    let num = 6;
    let divisor = 5;

    let result = num % divisor;
    println!("rest: {result}");

    let timestamp = 725469;

    let days = timestamp / 86400;
    let hours = (timestamp % 86400) / 3600;
    let minutes = (timestamp % 3600) / 60;
    let seconds = timestamp % 60;

    println!("{days}d {hours}h {minutes}m {seconds}s");

    let is_true = true;
    let is_true: bool = false;

    // the char type occupies 4 bytes
    let char = 'a'; // 4 bytes
    println!("unique char: {}", char);
    let char = 'ℤ'; // 4 bytes
    println!("symbol: {}", char);
    let char = '😈'; // 4 bytes
    println!("emoji: {}", char);

    // tuple
    let coord_2d = (0.0, 0.0);
    let coord_3d = (6.5, 5.9, 9.1);

    // destructuring
    let (x, y, z) = coord_3d;
    println!("coord: x: {x}, y: {y}, z: {z}");

    let x_coord = coord_3d.0;
    let y_coord = coord_3d.1;
    let z_coord = coord_3d.2;

    println!("again coord: x: {x_coord}, y: {y_coord}, z: {z_coord}");

    // Arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let message_event_buffer: [u8; 5] = [0x03, 0x55, 0x40, 0x03, 0x64]; // that goes into the stack
    println!("{:?}", message_event_buffer)


}
