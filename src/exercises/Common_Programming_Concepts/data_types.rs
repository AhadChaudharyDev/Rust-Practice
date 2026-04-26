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
}
