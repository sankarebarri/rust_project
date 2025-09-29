fn main() {
    let st = String::from("helloworld");
    let word = first_word(&st);
    // st.clear();
    println!("{word}");

    let string = String::from("Alpha Bravo");
    println!("{string}");
    let h = &string[0..5];
    let w = &string[6..11];
    println!("{h} - {w}");
    println!("{w}");

}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return i;
            return &s[0..i]
        }
    }
    // s.len()
    &s[..]
}

// fn range_method(string: &String)  {

// }