# rust-tutorial

## Description
### This repository is a study respository to follow Rust tutorial course.

### CHAPTER 1
1. Rust compiles with `rustc` e.g) `rustc main.rs`
2. Rust is implemented in `main()`
3. Compiled Rust file could be implemented anywhere without IDE not like Python, Javascript etc.
4. Compiled Rust file is binary file and it's so fast. Rust is developed with having purpose substitute for C++.
5. **Cargo.toml** is kind of **package.json** it deals with version and library.
> [Crates.io](https://crates.io/)
6. `cargo new PROJECT_NAME`: Make project and it has git itself.
> if you want override git use `cargo new --vcs=git` or no version control system by using the --vcs flag [help](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
7. `cargo build`: Build a debug project(When you add new **crate** you should build again).
8. `cargo check`: Check whether it can be compiled or not. (Use frequently this than using `build`).
9. `cargo build --release`: Release a project.
10. `rustup`: Install the latest stable version of Rust.

**CONVENTION**
```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

### CHAPTER 2
1. `use std::io` is like `import io from 'std';` 
2. `let`: variable, shadow .
> e.g) <br/>`let mut guess = something;`<br/>`let guess = something2`
3. `expect()`: Dealing with error.
4. Rust has strict check system.
5. `match`: Something like `switch`.
6. `loop`: Something like `while`.

### CHAPTER 3
1. `cargo new PROJECT_NAME --vcs=none` means that install a project without installing **.git** and **.gitignore**.
2. `const MAX_POINTS: u32 = 100_000` Upper case **Constants** name is recommended for readability. And you can use underscore between numeric literals.
3. `let x = 1; let x = x + 1;` when **Shadowing** is applied like this, `x` value will be immediately immutable after being computed. And we can mutate **type** which can not be done when use `let mut`.
4. Rust is *statically typed language*. So you should always use *annotation*. 
> e.g) `let guess: u32 = 42` 
5. Rust has four primary scalar types: *integers, floating-point numbers, Booleans, and characters*.

Length | Signed | Unsigned
------------ | ------------- | -------------
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch-bit | isize | usize

> **Signed** refers to that it is possible for the number to be negative. **Unsigned** refers to that only positive.
> Additionally, the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
6. Rust’s floating-point types are **f32** and **f64**, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. 
> e.g) `let x = 2.0; let y: f32 = 3.0;`
7. Booleans are *one byte* in size. The Boolean type in Rust is specified using bool. 
> e.g) `let t = true; let f: bool = false;`
8. **char** literals are specified with *single quotes* with *4 bytes*, as opposed to **string literals**, which use *double quotes*. 
> e.g) `let c = 'z'; let s = "Hello!";`
9. A **tuple** is a general way of grouping together a number of values with a variety of types into one compound type. *Tuples have a fixed length: once declared, they cannot grow or shrink in size*. 
> e.g) `let tup: (i32, f64, u8) = (500, 6.4, 1);`
> Destructure: `let tup = (500, 6.4, 1); let (x, y, z) = tup;`<br/>or<br/>`let x: (i32, f64, u8) = (500, 6.4, 1); let five_hundred = x.0;`
10. Unlike a tuple, every element of an **array** must have the *same type*. Arrays in Rust are *different from arrays in some other languages because arrays in Rust have a fixed length, like tuples*.<br/> Arrays are useful when you want your data allocated on the stack rather than the heap. 
> e.g) `let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];`
> With type: `let a: [i32; 5] = [1, 2, 3, 4, 5];`
> if you want to create an array that contains the same value for each element. `let a = [3; 5];` => result `[3, 3, 3, 3, 3];`
> If you put index out of bounds, The program exited with an error message and didn't execute the final println! statement. When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. This check has to happen at runtime, especially in this case, because the compiler can't possibly know what value a user will enter when they run the code later.
11. A **vector** is a similar collection type provided by the standard library *that is allowed to grow or shrink in size*. If you’re unsure whether to use an array or a vector, you should probably use a vector.
12. Rust code uses snake case as the conventional style for **function** and **variable names**. *In snake case, all letters are lowercase and underscores separate words*.
> e.g) `another_function();`
> With param: `fn another_function(x: i32) { println!("The value of x is: {}", x); }`
13. Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. *Calling a **function** is an expression. Calling a **macro** is an expression. The block that we use to create new scopes, **{}**, is an expression. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value*.
> e.g) `let y = { let x = 3; x + 1 };` It returns `4`.

## Site
[Rust Official](https://www.rust-lang.org/learn)