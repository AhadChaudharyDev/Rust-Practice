pub fn slice_type() {
    // ===============================
    // String (owned heap data)
    // ===============================
    let s = String::from("Hello This is Ahad chaudhary");

    // Find first word using string slice
    let f_word = first_word(&s);
    println!("first word = {f_word}");
}

// ---------------------------------
// Returns the first word of a string
// ---------------------------------
// Takes &str instead of &String so it works with:
// - String
// - &String
// - string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // Iterate over bytes with index
    for (index, item) in bytes.iter().enumerate() {
        // Check for space character
        if *item == b' ' {
            // Return slice from start to space
            return &s[..index];
        }
    }

    // If no space found, whole string is one word
    &s[..]
}
