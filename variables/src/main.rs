use std::io;

fn main() {
    // let x = 5; // immutable
    let mut x = 5; // mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}",THREE_HOURS_IN_SECONDS);

    // Shadowing
    let a = 5;

    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of x in the inner scope is: {a}")
    }

    println!("The value of x is: {a}");

    let spaces = "  ";
    // let spaces = spaces.len();

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat: char = '😻';

    // The Tuple Type

    let tup: (i32, f64, u8, char) = (700, 7.7, 7, 's');

    let (x, y, z, w) = tup; // deconstructing

    println!("The value of w is: {w}");

    let seven_hundred = tup.0;
    let s = tup.3;

    println!("{s}");

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

     let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
        
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}