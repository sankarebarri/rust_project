fn main() {
    let x = 5 + 3;
    let y = 10.5 - 6.0;
    let z = 4 * 30;

    let a = 56.7 / 32.2;
    let b = -5 / 3;
    let c = 56.0 / 32.0;
    let d = 43 % 5;

    let t = true;
    let f: bool = false;

    let e = 'z';
    let g: char = 'b';
    let heart_eyed_cat = '😻';

    println!("x={x}, y={y}, z={z}, a={a}, b={b}, c={c}, d={d}. {t}, {f}. {e}, {g}, {heart_eyed_cat}");

    // Compound types
    // THe tuple types
    let tup: (u32, f64, i8) = (65, 20.5, 6);
    let (x, y, z) = tup;
    println!("x={x}, y={y}, z={z}");

    // Accessing tuple elements with .
    let sixty_five = tup.0;
    let twenty_point_five = tup.1;
    let six = tup.2;
    println!("sixty_five={sixty_five}, twenty_point_five={twenty_point_five}, six={six}");

    // Arrays
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let first = arr[0];
    println!("first={first}");
    let arr = [1, 2, 3, 4, 5, 6];
    let last = arr[5];
    println!("last={last}");

    // initialising the same element
    let arr = [3; 5]; // [3, 3, 3, 3, 5]
    let second = arr[1];
    println!("second={second}");
}