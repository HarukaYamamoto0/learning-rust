#![allow(unused_variables)]

fn main() {
    print!("Hello, ");
    another_function();
    repeat(|| println!("repeated"), 99); // closure function

    let a = println!("a"); // ()
}

fn another_function() {
    print!("World\n");
}

fn repeat(block: fn(), amount: u32) {
    for i in 0..amount {
        block()
    }
}
