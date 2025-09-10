fn main() {
    println!("Hello, world!");

    another_function();
    let x = five();
    println!("Hello, {x}!");
    let y = plus_one(5);
    println!("Hello, plus_one {y}!");
}

fn another_function() {
    println!("This is the main another_function");
    function_with_paramter(3);
}

fn function_with_paramter(x: i32) {
    println!("x = {x}");
    function_with_multi_paramter(6, 'a');
}

fn function_with_multi_paramter(x: i32, h: char) {
    println!("The area of this is {x}.{h}");
    this_main();
}

fn this_main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}