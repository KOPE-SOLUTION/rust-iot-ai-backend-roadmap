fn main() {
    println!("=== Ownership & Borrowing Demo ===");

    // ===========================================
    // 1. Ownership Move
    // ===========================================
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2, s1
    // println!("s1: {}", s1); // This will cause a compile error because s1 is no longer valid
    println!("s2: {}", s2); // s2 owns the string now


    // ===========================================
    // 2. Clone (copy data)
    // ===========================================
    let a = String::from("World");
    let b = a.clone(); // a is cloned to b, both are valid
    println!("a: {}, b: {}", a, b);


    // ===========================================
    // 3. Borrowing (references)
    // ===========================================
    let text = String::from("rust");
    print_length(&text); // Borrowing text, not taking ownership
    println!("Text after function call: {}", text); // text is still valid

    // ===========================================
    // 4. Mutable Borrowing
    // ===========================================
    let mut msg = String::from("hello");
    change_text(&mut msg); // Borrowing mutably to change the text
    println!("Changed message: {}", msg); // msg is changed

    // ===========================================
    // 5. Borrowing Rules demo
    // ===========================================
    let mut data = String::from("data");
    let r1 = &data; // Immutable borrow
    let r2 = &data; // Another immutable borrow
    println!("r1: {}, r2: {}", r1, r2); // Both r1 and r2 can be used here
    // let r3 = &mut data; // This will cause a compile error because we cannot have a mutable borrow while immutable borrows exist


    // ===========================================
    // 6. Scope fix
    // ===========================================
    let mut value = String::from("ok");
    {
        let r = &value;
        println!("Inside inner scope: {}", r); // r is valid here
    }

    let r_mut = &mut value; // Now we can borrow mutably after the immutable borrow is out of scope
    r_mut.push_str("!"); // Modifying the value through mutable reference
    println!("Final value: {}", value); // value is changed
}

// ============================================
// Function to print the length of a string slice
// ============================================
fn print_length(s: &String) {
    println!("The length of '{}' is {}.", s, s.len());
}

// ============================================
// Function to change the text by borrowing mutably
// ============================================
fn change_text(s: &mut String) {
    s.push_str(" world"); // Modifying the string through mutable reference
}