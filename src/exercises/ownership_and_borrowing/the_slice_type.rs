pub fn slice_type() {
    // ===============================
    // String (owned heap data)
    // ===============================
    let s = String::from("Hello This is Ahad chaudhary");

    // Find first word using string slice
    let f_word = first_word(&s);
    println!("first word = {f_word}");

    // Find second word
    let s_word = second_word(&s);
    println!("second word = {s_word}");
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

// ---------------------------------
// Returns the second word of a string
// ---------------------------------
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_space = None;

    // Step 1: Find index of first space
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            first_space = Some(index);
            break;
        }
    }
    // If no space exists, there is no second word
    let start = match first_space {
        Some(i) => i + 1, // start after first space
        None => return "",
    };

    // Step 2: Find next space after first word
    for (index, &item) in bytes[start..].iter().enumerate() {
        if item == b' ' {
            // Slice from second word start to next space
            return &s[start..start + index];
        }
    }

    // If no second space found, return till end
    &s[start..]
}
