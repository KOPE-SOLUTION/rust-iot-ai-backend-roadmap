fn main() {
    println!("=== Rust Syntax Basics ===");

    // Variables
    let x = 10;
    let y: i32 = 20;

    println!("x: {}, y: {}", x, y);


    // Mutable variable
    let mut count = 0;
    count += 1;

    println!("Count: {}", count);


    // Function
    let sum = add(x, y);
    println!("Sum: {}", sum);


    // If condition
    if sum > 20 {
        println!("Sum is greater than 20");
    } else {
        println!("Sum is 20 or less");
    }


    // Loop
    for i in 0..5 {
        println!("Loop iteration: {}", i);
    }


    // Ownership example
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2

    // println!("s1: {}", s1); // This will cause a compile error because s1 is moved
    println!("s2: {}", s2);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
