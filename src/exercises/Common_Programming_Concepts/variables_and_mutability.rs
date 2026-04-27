pub fn variables() {
    // By default variable in rust are immutable
    // we cannot modify immutable variable
    // If we want to make it mut then we use (mut) keyword

    // Mutable variable
    println!("==========Mutable Variable==========");
    let mut _x = 10;
    _x = 100;
    println!("The Value of x is {}", _x);

    // Immutable variable
    println!("==========Immutable Variable==========");
    let y = 10;
    println!("The value of y is {}", y);

    // Shadowing

    println!("==========Shadowing==========");
    let z = 10;
    {
        let z = 'c';
        println!("the value of z in inner scope is {}", z);
    }
    println!("the value of z in outer scope is {}", z);

    // Constant

    println!("==========Constant==========");
    const VALUE_OF_PI: f64 = 3.1415;

    println!("The Value of pi is {}", VALUE_OF_PI);
}
