fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = " ";
    // Shadowing allows us to change the type of a variable
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");
}