pub fn borrowing() {
    println!("----------------");
    // =============================
    // Immutable borrowing
    // =============================

    // The String "hello" is stored on the heap
    // Variable `s` is the OWNER of this String
    let s = String::from("hello");

    // We pass only a REFERENCE to the function
    // Ownership of the String stays with `s`
    let s1 = calculate_length(&s);

    // We can still use `s` here
    // because ownership was not moved, only borrowed
    println!("The length of {s} is {s1}");

    println!("----------------");
    // =============================
    // Mutable references
    // =============================

    let mut s2 = String::from("Ahad Chaudhary, ");

    // Passing a MUTABLE reference
    let s3 = modify_borrowing_ref(&mut s2);

    println!("Mutable reference result: {s3}");

    // =============================
    // Only ONE mutable reference at a time
    // =============================

    let mut x = String::from("Hello");

    let _x1 = &mut x;
    // let x2 = &mut x; // ❌ NOT allowed: second mutable borrow

    // println!("x1 = {x1}, x2 = {x2}");

    // Rust prevents data races at compile time
    // We also cannot have mutable and immutable references at the same time

    println!("----------------");

    // =============================
    // Multiple mutable references using scopes
    // =============================

    let mut x = String::from("Hello");

    {
        let x2 = &mut x;
        println!("x2 = {x2}");
    } // x2 goes out of scope here (mutable borrow ends)
    
    let x1 = &mut x; // ✅ now allowed
    println!("x1 = {x1}");
}

// =============================
// Borrowing (immutable reference)
// =============================

fn calculate_length(cal: &String) -> usize {
    // `cal` is a borrowed reference
    // This function does NOT own the String
    let length = cal.len();

    // Only the length is returned
    // The String is NOT dropped here
    length
}

// =============================
// Borrowing (mutable reference)
// =============================

fn modify_borrowing_ref(some_string: &mut String) -> &String {
    // Because we have a MUTABLE reference,
    // we ARE allowed to modify the borrowed value
    some_string
        .push_str("God willing, if I keep working like this, I will become a great developer");

    // Returning an immutable reference to the same String
    some_string
}
