fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    print_shadowed_variable();
    print_data_types();
}

fn print_shadowed_variable() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");
}

fn print_data_types() {
    let x = 2.0; // f64
    println!("The value of x float is: {x}");
    let x: f32 = 3.0; // f32
    println!("The value of x float is: {x}");

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
    println!("The value of sum is: {sum}");
    println!("The value of difference is: {difference}");
    println!("The value of product is: {product}");
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    println!("The value of remainder is: {remainder}");

    // tuples
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    println!("The value of tup is: {0}", tup.2);

    //arrays
    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July"];
    println!("The value of a is: {0}", a[0]);
    println!("The value of months is: {0}", months[0]);

    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

    println!("The value of a is: {0}", a[0]);

    if a[0] < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    print_loops_with_label();
    print_conditional_loops();
    print_loop_with_for();
}

fn print_loops_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn print_conditional_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn print_loop_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    let range = 1..4;
    for number in range.rev() {
        println!("{number}!");
    }
}