fn main() {
    test_heap();
    deep_copy();
    integer_copy();
}

fn test_heap() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("Hey");
    let mut s2 = s1;
    s2.push_str(" man");

    println!("{}, you!", s2);
}

fn deep_copy() {
    let s1 = String::from("HA!");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1,s2);
}

// ? There is no difference whether clone() or not.
fn integer_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}