// fn main() {
//     let mut s = String::from("Hello");
//     s.push_str(", world!");
//     println!("{s}")
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, world!");
// }

// fn main() {
//     let mut s = String::from("hello");
//     s = String::from("ahoy");
//     println!("{s}, world!");
// }

// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();
//     println!("s1={s1}, s2={s2}");
// }

// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");

// }

// fn main() {
//     let s = String::from("hello");
//     println!("{s}");
//     takes_ownership(s);
//     // println!("{s}");
//     let x = 5;
//     makes_copy(x);
//     println!("{x}");
// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

// fn main() {
//     let s1 = gives_ownership();
//     println!("{s1}");

//     let s2 = String::from("hello");
//     println!("s2={s2}");


//     let s3 = takes_and_gives_back(s2);
//     println!("s1={s1}");
//     println!("s3={s3}");
//     // println!("s2={s2}"); won't work because the s2 vaue has been moved into s3
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);    

    println!("The length of '{s2}' is {len}");
    println!("s1={s1}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}