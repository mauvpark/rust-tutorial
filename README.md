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
> If you put index out of bounds, The program exited with an error message and didn't execute the final println! statement.<br/>When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length.<br/>This check has to happen at runtime, especially in this case, because the compiler can't possibly know what value a user will enter when they run the code later.
11. A **vector** is a similar collection type provided by the standard library *that is allowed to grow or shrink in size*. If you’re unsure whether to use an array or a vector, you should probably use a vector.
12. Rust code uses snake case as the conventional style for **function** and **variable names**. *In snake case, all letters are lowercase and underscores separate words*.
> e.g) `another_function();`
> With param: `fn another_function(x: i32) { println!("The value of x is: {}", x); }`
13. Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. *Calling a **function** is an expression.<br/>Calling a **macro** is an expression.<br/>The block that we use to create new scopes, **{}**, is an expression.<br/>Expressions do not include ending semicolons.<br/>If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value*.
> e.g) `let y = { let x = 3; x + 1 };` It returns `4`.
14. You must be explicit and always provide **if** with a Boolean as its condition. 
> Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.<br/>Because if is an expression, we can use it on the right side of a let statement.<br/>e.g) `let number = if condition { 5 } else { 6 };`<br/>But if return types are different like `let number = if condition { 5 } else { "six" };` will fire type error.
15. Rust has three kinds of loops: **loop**, **while**, and **for**.

### CHAPTER 4
1. All data stored on the **stack** must have a known, ***fixed size***. 
> Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
2. Data with an ***unknown size*** at compile time or a size that might change must be stored on the **heap** instead.
> ***Allocating on the heap***: The heap is less organized. When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. Allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.<br/><br/>Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
3. **Ownership Rules**
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
4. **String literal**, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program. <br/>
- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String. **GC(Garbage Collector)**
5. `drop`, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.*(= RAII)*
6. the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.<div style="display:block"><img src="https://doc.rust-lang.org/book/img/trpl04-02.svg"></div> This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
> To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:
7. If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a **move**<div style="display:block"><img src="https://doc.rust-lang.org/book/img/trpl04-04.svg"></div>***s1 was moved into s2***.
8. Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
9. Types such as **integers** that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
10. **Copy**
- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating point types, such as f64.
- The character type, char.
- Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Site
[Rust Official](https://www.rust-lang.org/learn)