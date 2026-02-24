// *** Ownership Rules ***
// - Each value in Rust has an owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {
    // *** String literal ***
    // - The actual string data is known at compile time and hardcoded in the program’s binary (read-only memory).
    // - `literal` is a mutable variable holding a reference (&'static str) to that data, stored on the stack.
    // - The data itself is immutable: we cannot change the contents of "hello".
    // - Yet the variable is mutable, so we can make it point to another string literal if we want.
    let mut literal: &'static str = "hello";
    literal = "world"; // Allowed; Move the reference to a different literal

    // *** String ***
    // - We need to allocate an amount of memory on the heap.
    // - Mutable owner + mutable data → we can modify the contents.
    let mut s = String::from("hello");
    s.push_str(", world!");

    // Rust never automatically creates a deep copy, cloning would be with the clone method: let s2 = s.clone(); 
    // This would create a deep copy of the heap data, allowing both s and s2 to be valid and independent.
    let s2 = s; // Both s2 and s point to the same heap data, but rust calls drop on s and cleans up the heap memory. 
    // --> s is now invalid and cannot be used.

    // *** Copy Trade ***
    // Both x and y are valid and independent because integers implement the so-called Copy trait, which means they are copied rather than moved.
    // integers, bolean, chat ... those types implement Copy
    // This is because integers are stored on the stack and have a known, fixed size, so copying them is efficient and does not involve heap allocation.
    let x = 5;
    let y = x; 

    // *** Application to functions ***
    // take_ownership(s);   // s is moved into the function and becomes invalid here
    // make_copy(x);        // x is copied into the function, so it remains valid here
    // let s3 = take_and_give_back(s); // s is moved into the function and becomes invalid here, but s3 is valid and owns the returned value

    // Prints the variables
    println!("{literal}");  // prints: world
    println!("{s2}");       // prints: hello, world!
    println!("Copy trade: x = {x}, y = {y}"); // prints: Copy trade: x = 5, y = 5
}