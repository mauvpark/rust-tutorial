use std::io;

fn main() {
    let mut x = 5; // ! let x = 5; invokes error!
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // Invalid Array Element Access
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
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // Expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    
    let x: u32 = expression_function(5);
    println!("The value of x is {} ", x);
}

fn expression_function (x: u32) -> u32 {
    x + 1
}
