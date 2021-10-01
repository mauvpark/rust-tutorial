fn main() {
    let s1 = String::from("hello");
    let mut s = String::from("hey");
    

    // ? String ownership will be moved into calculate_length() if you do not use '&'.
    // ? As borrowing, '&: references' enables to use variable without taking ownership.
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
    
    // ! But mutable references have one big restriction: 
    // ! you can have only one mutable reference to a particular piece of data in a particular scope. 
    // * Workaround
    { 
        let r2 = &mut s;
    }
    change(&mut s);

    // ! Users of an immutable reference don’t expect the values to suddenly change out from under them!
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // ! let r3 = &mut s; // BIG PROBLEM

    // ! println!("{}, {}, and {}", r1, r2, r3);

    // * Workaround
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

  fn no_dangle() -> String {
    let s = String::from("hello");

    s
}