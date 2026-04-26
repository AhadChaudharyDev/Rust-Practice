pub fn start() {
    // ================================================
    // Scalor types
    // =================================================

    println!("===============================");

    // =================================================
    // Integer type inference
    // =================================================

    println!("Integer type inference");

    // If we don't specify a type, Rust infers it.
    // By default, integer literals are inferred as i32.

    let x = 10;
    println!("x = {x}");

    // =================================================
    // Integer overflow (u8 example)
    // =================================================

    println!("Integer overflow (u8)");

    // u8 can store values from 0 to 255.
    // Adding 1 to 255 causes overflow.
    //
    // - Debug mode  -> panic (overflow check ON)
    // - Release mode -> wraps around (overflow check OFF)

    let y: u8 = 255;
    // y = y + 1; // Uncomment to see panic in debug mode
    println!("y = {y}");

    // =================================================
    // Explicit overflow handling
    // =================================================

    println!("Explicit overflow handling");

    println!("-----------------");

    // 1️⃣ Wrapping arithmetic (wraps around)
    let z: u8 = 255;
    let z = z.wrapping_add(1);
    println!("wrapping_add: {z}");

    println!("-----------------");

    // 2️⃣ Checked arithmetic (returns Option)
    let c: u8 = 255;
    let c = c.checked_add(1);
    println!("checked_add: {:?}", c); // None on overflow

    println!("-----------------");

    // 3️⃣ Overflowing arithmetic (returns value + overflow flag)
    let d: u8 = 255;
    let d = d.overflowing_add(1);
    println!("overflowing_add: {:?}", d); // (0, true)

    println!("-----------------");

    // 4️⃣ Saturating arithmetic (clamps to max value)
    let e: u8 = 255;
    let e = e.saturating_add(1);
    println!("saturating_add: {e}"); // stays at 255

    println!("-----------------");

    println!("Type conversion & static typing");

    // Rust is a statically typed language.
    // If the compiler cannot infer the type, we must specify it explicitly.
    // Here, "42" is a &str, and we convert it into a u32.

    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess = {guess}");

    println!("===============================");
    println!("Float type inference");

    // Rust has two floating-point types: f32 and f64
    // Default floating-point type is f64

    let a = 12.5;
    let b = 12.52;
    let result = a + b;

    println!("result = {result}");

    // =================================================
    // Boolean
    // =================================================

    println!("Boolean type inference");

    let t = true; // implicit type
    let u: bool = false; // explicit type

    println!("t = {t}");
    println!("u = {u}");

    // =================================================
    // Character
    // =================================================

    println!("Character type inference");

    // char is a Unicode scalar value
    // It always takes 4 bytes

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {c}");
    println!("z = {z}");
    println!("heart_eyed_cat = {heart_eyed_cat}");

    // ================================================
    // Compound types
    // =================================================

    // tuple

    let tup = (1, 'A', "Rustacean", 10.5);

    // indexig

    println!("value = {}", tup.0);
    println!("value = {}", tup.1);
    println!("value = {}", tup.2);
    println!("value = {}", tup.3);

    // destructuring

    let (a, b, c, d) = (1, 'A', "Rustacean", 10.5);

    println!("value = {}", a);
    println!("value = {}", b);
    println!("value = {}", c);
    println!("value = {}", d);
}
