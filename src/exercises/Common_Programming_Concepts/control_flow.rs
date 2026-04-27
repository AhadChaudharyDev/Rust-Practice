pub fn control_flow() {
    // =============================
    // if statement
    // =============================

    println!("if condition");

    let x = 10;
    if x < 5 {
        println!("The value of x is less than 5");
    } else {
        println!("The value of x is greater than 5");
    }

    println!("------------------");
    // =============================
    // else if (multiple conditions)
    // =============================

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is divisible by 1");
    }

    println!("------------------");

    // =============================
    // if as an expression (returns value)
    // =============================
    let condition = true;

    let value = if condition { 5 } else { 6 };
    //  if expression return value

    println!("value = {value}");

    println!("------------------");

    // =============================
    // loop (infinite loop)
    // =============================
    loop {
        println!("AhadChaudhary");
        break; // stop loop
    }

    println!("------------------");

    // =============================
    // loop returning a value
    // =============================

    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        counter += 1;
    };

    println!("result = {result}");

    println!("------------------");

    // =============================
    // while loop
    // =============================

    let mut num = 0;

    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("Liftoff!!!");

    println!("--------------------------");

    // =============================
    // for loop with enumerate
    // =============================

    let arr = [1, 2, 3, 4, 5];

    for (index, value) in arr.iter().enumerate() {
        println!("Value on index {} is {}", index, value);
    }

    println!("--------------------");

    // =============================
    // for loop with range
    // =============================

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
