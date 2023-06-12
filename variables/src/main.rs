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

    let (x, y, z, w, q) = tup; // deconstructing

    println!("The value of w is: {w}");

    let seven_hundred = tup.0;
    let s = tup.3;

    println!("{s}");
}