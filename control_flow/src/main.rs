fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;

    // Give the outer loop the label 'counting_up
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // exit inner loop
                break;
            }
            if count == 2 {
                // exit outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("List: {element}");
    }

    for number in (1..6).rev() {
        println!("{number}.");
    }
    println!("LIFTOFF !!");
}