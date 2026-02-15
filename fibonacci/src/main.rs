use std::io;

fn fibonacci(n: &u32) -> u32 {
    if *n == 0 {
        0
    } else if *n <= 2 {
        1
    } else {
        fibonacci(&(*n - 2)) + fibonacci(&(*n - 1))
    }
}

fn main() {

    let mut input = String::new();

    println!("Please type in a number");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    println!("Your input: {input}");
    let result = fibonacci(&input);
    println!("Fibonacci result: {result}")

}
