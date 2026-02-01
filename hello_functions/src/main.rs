fn main() {
    another_function(5);
    print_labeled_measurement(10, 'm');

    // calling a function is an expression
    statements_and_expressions();
    // calling a macro is also an expression
    println!("This is a macro call");

    // Using a function with a return value
    println!("The value returned from function five() is: {}", five());
    println!("The value returned from function plus_one(10) is: {}", plus_one(10));
}

// A function that takes a single parameter
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// A function that takes multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function bodies are made up of statements, optionally ending with an expression
// Rust is an expression-based language
fn statements_and_expressions() {
    // `let` bindings are statements and do not return values; 
    // { let x = 3; x + 1 } is an expression that returns the value of the final expression in the block
    // Statements do not return values, thus their is no assignment of a let statement to a variable
    // This would be invalid: let x = (let y = 6); --> Again, let statements do not return values
    // In C or Ruby, this would be valid: x = y = 6 and both x, y would be assigned the value 6
    let y = {
        // This is a block expression
        let x = 3; // statement
        x + 1      // expression (no semicolon → returned from the block)
    };
    // println! is a macro that prints to the console
    println!("The value of y is: {y}");
}

// Functions with return values
// Return values are not need to be named; but must be declared with the -> syntax
// The return value of the function is the value of the final expression in the function body
// !! Do not put a semicolon at the end of the expression you want to return !!
fn five() -> i32 {
    5 // no semicolon means this is an expression that returns the value 5
    // 5; would be fail because it would be a statement, not an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1 // returns the value of x + 1
}