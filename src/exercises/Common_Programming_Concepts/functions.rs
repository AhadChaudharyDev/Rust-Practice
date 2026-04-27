pub fn fun() {
    // ==================================={}
    // Statements and Expressions
    // ===================================

    // Rust is an expression-based language

    // let x = y = z = 10;
    // Not allowed because assignment is a statement in Rust
    // unlike C/C++ and Ruby

    let y = {
        let x = 10;
        x + 1 // last expression (no semicolon) returns the value
    };

    println!("y = {y}");

    println!("-----------------");

    // ===================================
    // Parameters
    // ===================================

    println!("Hello");
    another_function("developers");

    println!("----------------");
}

fn another_function(x: &str) {
    println!("Rustaceans {}!!!", x);
}
