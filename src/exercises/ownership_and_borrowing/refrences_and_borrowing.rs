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
