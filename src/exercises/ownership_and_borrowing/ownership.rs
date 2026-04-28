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

    println!("------------------------");
    // ===============================
    // Return Values and Scope
    // ===============================

    let s7 = gives_ownership();

    println!("Give ownership = {s7}");

    let s7 = String::from("what's up?");

    let s8 = takes_and_gives_back(s7);

    println!("Gives and take back ownership = {s8}");

    println!("----------------------");

    // =============================
    // Return multiple value using tuple
    // =============================

    let d = String::from("AadChaudhary");

    let (d1 , len) = cal_len(d);

    println!("The length of '{d1}' is {len}.");
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

// =================================
// Function that gives ownership
// =================================

// This function:
// - creates a String on the heap
// - returns it to the caller
// - ownership is transferred to the caller

fn gives_ownership() -> String {
    let s7 = String::from("Heyyyy");

    // Returning s7 moves ownership to the caller
    s7
    // No drop here because ownership is moved
}

// This function:
// - takes ownership of a String as a parameter
// - returns the same String back
// - ownership moves in and then moves out

fn takes_and_gives_back(a_string: String) -> String {
    // a_string owns the heap memory here

    a_string
    // Ownership is moved back to the caller
}

fn cal_len(d: String)->(String,usize){
    
    let length = d.len();

    (d , length)

}