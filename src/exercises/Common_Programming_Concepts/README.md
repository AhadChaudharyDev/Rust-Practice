## 📂 Rust Topics

| Topic | File |
|------|------|
|
| Hello_World | [Open](src/exercises/common_programming_concepts/hello-world.rs) |
| Variables_And_Mutables | [Open](src/exercises/common_programming_concepts/variables_and_mutability.rs) |
| Data_Types | [Open](src/exercises/common_programming_concepts/data_types.rs) |
| Functions | [Open](src/exercises/common_programming_concepts/functions.rs)|
| Control_Flows | [Open](src/exercises/common_programming_concepts/control_flows.rs) |




# Rust Variables & Memory Concepts Demo

This project demonstrates fundamental Rust concepts including **mutable variables, immutable variables, shadowing, and constants** using simple examples.

---

## 📌 Overview

Rust is a systems programming language focused on safety and performance. One of its core principles is **immutability by default**, which helps prevent bugs and ensures memory safety.

This project illustrates:

- Mutable variables (`mut`)
- Immutable variables
- Variable shadowing
- Constants (`const`)

---

## 🧠 Concepts Covered

### 1. Immutable Variables

By default, variables in Rust are immutable (cannot be changed after assignment).

```rust
let y = 30;
println!("The value of y is = {y}");
```

### 2. Mutable Variables

To modify a variable, we use the mut keyword.

```rust
let mut x = 10;
x = 20;
println!("The value of x is = {x}");
```

### 3. Shadowing

Shadowing allows us to redefine a variable with the same name. Each new binding creates a new variable.

```rust
let mut a = 10;
a += 1;

{
    let a = 20;
    let a = a * 20;
    println!("Inner scope value = {a}");
}
```
👉 Inner scope variables do not affect outer scope variables.

### 4. Constants

Constants are always immutable and must have a type annotation. They are evaluated at compile time.

```rust
const VALUE_OF_PI: f64 = 3.14159;
println!("The value of PI is = {VALUE_OF_PI}");
```

-------

# 📊 Rust Data Types & Type System

This project demonstrates core concepts of Rust's **type system**, including scalar types, compound types, type inference, and safe handling of integer overflow.

---

## 🚀 Overview

Rust is a **statically typed language**, which means every variable must have a known type at compile time. This example covers how Rust:

* Infers types automatically
* Requires explicit type annotations when needed
* Handles integer overflow safely
* Works with scalar and compound data types

---


## 🧠 Concepts Covered

### 🔹 Type Conversion & Static Typing

Rust enforces strict type safety. When the compiler cannot infer a type, you must specify it manually.

```rust
let guess: u32 = "42".parse().expect("Not a number");
```

---

### 🔹 Integer Type Inference

Rust automatically infers integer types. By default, integers are `i32`.

```rust
let x = 10;
```

---

### 🔹 Integer Overflow

Rust prevents unsafe overflow:

* **Debug mode** → panic (error)
* **Release mode** → wraps around

```rust
let y: u8 = 255;
// y = y + 1; // causes panic in debug mode
```

---

### 🔹 Safe Overflow Handling Methods

Rust provides multiple safe ways to handle overflow:

| Method            | Behavior                         |
| ----------------- | -------------------------------- |
| `wrapping_add`    | Wraps around (255 → 0)           |
| `checked_add`     | Returns `None` on overflow       |
| `overflowing_add` | Returns `(value, overflow_flag)` |
| `saturating_add`  | Stays at max value               |

---

### 🔹 Floating-Point Types

Rust supports:

* `f32`
* `f64` (default)

```rust
let result = 12.5 + 12.52;
```

---

### 🔹 Boolean Type

```rust
let t = true;
let u: bool = false;
```

---

### 🔹 Character Type

* Unicode supported
* Always 4 bytes

```rust
let c = 'z';
let emoji = '😻';
```

---

## 📦 Compound Types

### 🔸 Tuple

* Stores multiple types
* Fixed size

```rust
let tup: (&str, f64, u8) = ("Ahad", 6.4, 1);
```

Access methods:

* Destructuring
* Indexing (`tup.0`)

---

### 🔸 Array

* Fixed size
* Same data type

```rust
let mut arr = [1, 2, 3, 4, 5];
```

#### Slicing

```rust
let slice = &arr[1..=4];
```

#### Initialize with same value

```rust
let arr2 = [5; 10];
```

---

# 🧠 Rust Statements, Expressions & Functions

This project demonstrates key Rust concepts including **statements vs expressions, function parameters, and return values**. It highlights how Rust differs from other languages like C/C++ by being an **expression-based language**.

---

## 🚀 Overview

Rust treats most constructs as **expressions**, meaning they return values. This allows writing concise and powerful code.

This example covers:

* Statements vs Expressions
* Expression blocks
* Function parameters
* Returning values (implicit & explicit)
* Tuples and type transformations

---



## 🧠 Concepts Covered

### 🔹 Statements vs Expressions

* **Statements** → perform actions but do NOT return values
* **Expressions** → evaluate to a value

❌ This is NOT allowed in Rust:

```rust
let x = y = z = 10;
```

✔ Because assignment is a **statement**, not an expression

---

### 🔹 Expression Block

Rust allows blocks `{}` to act as expressions:

```rust
let y = {
    let x = 10;
    x + 1
};
```

👉 The last line (without `;`) is returned

---

### 🔹 Functions with Parameters

Rust functions can take multiple parameters with explicit types:

```rust
fn another_function(age: i32, note: &str)
```

---

### 🔹 Function Returning Tuple

```rust
fn para_demo(a: i32, b: char, c: f64, d: bool) -> (String, char, f64, i32)
```

Inside this function:

* Integer → String conversion
* Character manipulation
* Float arithmetic
* Boolean → Integer using expression

---

### 🔹 Returning Values (Implicit Return)

Rust functions return values without using `return` keyword:

```rust
fn ret_value(x: i32) -> i32 {
    x + 1
}
```

👉 No semicolon = return value

---

# 🔁 Rust Control Flow (if, loops & iteration)

This project demonstrates Rust’s **control flow mechanisms**, including conditional statements and different types of loops.

It covers how Rust handles:

* `if` / `else if` conditions
* `if` as an expression
* `loop`, `while`, and `for` loops
* Returning values from loops
* Loop labels for advanced control

---

## 🚀 Overview

Control flow allows you to control how your program executes based on conditions and repetition.

Rust provides powerful and safe constructs for:

* Decision making
* Iteration
* Breaking and controlling loops

---

## 🧠 Concepts Covered

### 🔹 If Statement

Basic conditional execution:

```rust id="l7l1ai"
if x <= 10 {
    println!("condition was true");
} else {
    println!("condition is false");
}
```

---

### 🔹 Else If (Multiple Conditions)

```rust id="3t3rbo"
if number % 4 == 0 {
    ...
} else if number % 3 == 0 {
    ...
} else if number % 2 == 0 {
    ...
} else {
    ...
}
```

👉 Used when multiple conditions need to be checked

---

### 🔹 If as Expression

Rust allows `if` to return values:

```rust id="1o5wph"
let value = if condition { 5 } else { 6 };
```

---

### 🔁 Loop (Infinite Loop)

```rust id="x36c4h"
loop {
    println!("Hello");
    break;
}
```

👉 Runs forever unless stopped using `break`

---

### 🔁 Loop Returning Value

```rust id="4r5o9z"
let result = loop {
    if counter == 10 {
        break counter * 2;
    }
};
```

👉 `break` can return a value from a loop

---

### 🔁 While Loop

```rust id="b8wqfy"
while num != 0 {
    println!("{num}");
    num -= 1;
}
```

👉 Runs while condition is true

---

### 🔁 For Loop (Array Iteration)

```rust id="m5q93p"
for (index, value) in arr.iter().enumerate() {
    println!("{index} -> {value}");
}
```

👉 Clean and safe iteration over collections

---

### 🔁 For Loop with Range

```rust id="y0qzbg"
for number in (1..4).rev() {
    println!("{number}");
}
```

👉 Iterates over a range (reverse order here)

---

### 🔁 Loop Labels (Advanced)

```rust id="6zjz0l"
'counting_up: loop {
    loop {
        if count == 2 {
            break 'counting_up;
        }
    }
}
```

👉 Allows breaking out of **outer loops**

---