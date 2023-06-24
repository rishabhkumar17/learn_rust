fn main() {
    // Repeating Code with loop

    // loop {
    //     println!("again!");
    // }

    // Returning Values from Loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
