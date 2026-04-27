pub fn fun() {
    // ==================================={}
    // Statements and Expressions
    // ===================================

    // Rust is an expression-based language

    // let x = y = z = 10;
    // Not allowed because assignment is a statement in Rust
    // unlike C/C++ and Ruby

    println!("Statements and Expressions");

    let y = {
        let x = 10;
        x + 1 // last expression (no semicolon) returns the value
    };

    println!("y = {y}");

    println!("-----------------");

    // ===================================
    // Parameters
    // ===================================

    println!("Parameters");

    println!("Hello");
    another_function("developers");

    println!("----------------");

    // ===================================
    // Function with return values
    // ===================================

    println!("Function with return values");

    let tup = para_demo(1, 1.5, 'c', "Ahad".to_string(), true);

    println!("{} {} {} {} {}", tup.0, tup.1, tup.2, tup.3, tup.4);
}

fn another_function(x: &str) {
    println!("Rustaceans {}!!!", x);
}

fn para_demo(a: i32, b: f64, c: char, mut d: String, e: bool) -> (i32, f64, char, String, i32) {
    let a = a + 1;
    let b = b + 10.5;
    let c = (c as u8 + 1) as char; // next character
    d.push_str("Chaudhary");
    let e = if e { 1 } else { 0 };

    (a, b, c, d, e) // tuple return (expression)
}
