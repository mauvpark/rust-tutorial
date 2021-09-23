# rust-tutorial

## Description
### This repository is study respository to follow Rust tutorial course.

## 2021.09.21
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

## 2021.09.22
### CHAPTER 3
1. `cargo new PROJECT_NAME --vcs=none` means that install a project without installing **.git** and **.gitignore**.
2. `const MAX_POINTS: u32 = 100_000` Upper case **Constants** name is recommended for readability. And you can use underscore between numeric literals.
3. `let x = 1; let x = x + 1;` when **Shadowing** is applied like this, `x` value will be immediately immutable after being computed. And we can mutate **type** which can not be done when use `let mut`.

## Site
[Rust Official](https://www.rust-lang.org/learn)