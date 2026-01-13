fn main() {
    // Integer types
    let i32: i32 = 42; // 32-bit signed integer
    let u64: u64 = 100; // 64-bit unsigned integer

    // Floating-point types
    let f32: f32 = 3.14; // 32-bit floating point
    let f64: f64 = 2.718281828459045; // 64-bit floating point

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean values
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Character type
    let c='c';
    let z: char = 'z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Tuples
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Arrray
    let a = [1, 2, 3, 4, 5];
    let b = [3; 5]; // an array of 5 elements, each initialized to 3



    println!("Integer i32: {}, u64: {}", i32, u64);
    println!("Floating-point f32: {}, f64: {}", f32, f64);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Truncated division: {}", truncated);
    println!("Remainder: {}", remainder);
    println!("Boolean t: {}, f: {}", t, f);
    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
    println!("Tuple values: {}, {}, {}", five_hundred, six_point_four, one);
    println!("Array: {:?} {:?}", a, b);
}