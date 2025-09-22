// fn main() {
//     let s1 = String::from("hello");
//     let len = length_of_word(&s1);

//     println!("The length of '{s1}' is {len}");
//     println!("{s1}");
// }
// fn length_of_word(s: &String) -> usize {
//     s.len()
// }


// This won't work because we're trying to change the borrowed value
// fn main() {
//     let s = String::from("Hello");

//     change(&s);
// }

// fn change(s: &String) {
//     s.push_str(", world");
// }

// How to mutate a refrences
fn main() {
    let mut s = String::from("Hello");

    change (&mut s);
    println!("{s}")
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}