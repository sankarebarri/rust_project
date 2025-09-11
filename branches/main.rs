fn main() {
    let number = 3;

    if number < 1 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    divisible_by_four();
    if_let_statement();
}

fn divisible_by_four() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let_statement() {
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is {number}");
}