fn main() {
    // let x = 5; // immutable
    let mut x = 5; // mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}",THREE_HOURS_IN_SECONDS);
}