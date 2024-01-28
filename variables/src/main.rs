use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Enter an array index");

    // Creates the empty mutable string
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index) // Scanes line and returns a result enum
        .expect("Failed to read line"); // Runs is the enum if Err
                                        // Reassign index to an int
    let index: usize = index.trim().parse().expect("Not a number");

    println!("The value fo the element at index {index} is: {}", a[index]);
}
