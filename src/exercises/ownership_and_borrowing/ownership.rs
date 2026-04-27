pub fn ownership() {
    // =========================================
    // String literal (&str)
    // =========================================
    // &str is a string slice (reference to data)
    // String literals are:
    // - immutable
    // - stored in the binary (read-only memory)
    // - very fast and lightweight

    let s = "hello"; // valid for the entire scope of main

    {
        let s = "world"; // valid only inside this block
        println!("inner scope {s}");
    } // s goes out of scope here

    println!("outer scope {s}");

    println!("-------------------");

    // =========================================
    // String type (heap allocated)
    // =========================================
    // String:
    // - stored on the heap
    // - mutable
    // - growable

    let mut v = String::from("Ahad");
    v.push_str("Chaudhary");

    println!("{v}");

    println!("-----------------");

    // =========================================
    // Copy (stack-only data)
    // =========================================
    // Integers implement the Copy trait
    // Assignment creates a new copy of the value

    let mut x=5;
    let y=x;

    x=10;

    println!("value of x is {}",x);
    println!("value of y is {}",y);

    println!("----------------");
}
