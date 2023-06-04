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

    println!("The value of x is: {a}")

}