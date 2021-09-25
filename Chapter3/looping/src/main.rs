fn main() {
    println!("Hello, world!");

    let counter = looping();
    println!("The value is {}", counter);
    let number = while_looping();
    println!("The last number is {}", number);
    improved_while();
    array_looping();
    let number = fibo_numbers(8);
    println!("That fibonacci number is {}", number);
}

fn looping () -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        } 
    }
}

fn while_looping () -> i32 {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
    number
}

fn improved_while () {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("SECOND LIFTOFF!!!");
}

fn array_looping () -> u32 {
    let a = [10, 20, 30, 40, 50];
    let mut val = 0;

    for element in a.iter() {
        println!("The value is: {}", element);
        val = *{element};
    };

    val
}

fn fibo_numbers (mut x: u32) -> u32 {
    let mut present = 1;
    let mut before = 0;
    while x > 0 {
        present = present + before;
        before = present - before;
        x -= 1;
    }

    present
}