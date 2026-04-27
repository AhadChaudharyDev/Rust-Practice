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

    let mut x = 5;
    let y = x;

    x = 10;

    println!("value of x is {}", x);
    println!("value of y is {}", y);

    println!("----------------");

    // =========================================
    // Move ownership (heap data)
    // =========================================
    // String does NOT implement Copy
    // Ownership is moved instead of copied

    let s1 = String::from("Rustacean");
    let s2 = s1; // ownership moves from s1 to s2

    println!("{s2}");
    // println!("{s1}"); // ❌ error: value used after move

    println!("--------------------");
    // =========================================
    // Reassignment and drop
    // =========================================
    // When a new value is assigned:
    // - old heap memory is freed
    // - new heap memory is allocated

    let mut _s3 = String::from("hello");
    _s3 = String::from("ahoy"); // "hello" memory is dropped here

    println!("{_s3}, world");

     println!("--------------------");

    // =========================================
    // Clone (deep copy)
    // =========================================
    // clone():
    // - copies heap data
    // - both variables own separate memory
    // - more expensive than Copy

    let s4 = String::from("Hello");
    let s5 = s4.clone();

    println!("s4 = {s4} and s5 = {s5}");

    println!("--------------------");

    // ===============================
    // Ownership and functions
    // ===============================

    // String type:
    // - stored on the heap
    // - owns its data
    // - does NOT implement Copy

    let s6 = String::from("AHadChaudhary!!");

    // Ownership of s6 MOVES into the function
    // After this call, s6 is no longer valid in main
    takes_ownership(s6);

    // println!("{s6}"); // ❌ error: value used after move


    // String literal:
    // - type is &str (string slice)
    // - immutable
    // - does NOT own the data
    // - implements Copy

    let a = "Alhumdulillah";

    // a is a &str (reference), so it is COPIED
    // Ownership does NOT move
    makes_copy(a);

    // a is still valid here
}

// =================================
// Function that takes ownership
// =================================

fn takes_ownership(some_string: String) {
    // some_string now OWNS the heap memory
    println!("Take ownership = {some_string}");
}
// some_string goes out of scope here
// drop() is called and heap memory is freed


// =================================
// Function that borrows a string slice
// =================================

fn makes_copy(string_literal: &str) {
    // string_literal is a borrowed reference
    // No ownership, no heap cleanup needed
    println!("Copy = {string_literal}");
}

