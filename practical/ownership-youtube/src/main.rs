//----------- Ownership rules -----
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    // s is not valid here, it's not yet declared
    let s = String::from("hello"); // s is valid from this point onwards
    // do stuff with s

    // takes_ownership(s);  // Ownership of `s` is moved to this function
    // println!("{}", s); // Error! `s` is no longer valid here

    let x = 5; // i32 is Copy, so it’s okay to use it later
    makes_copy(x);
    println!("{}", x); // `x` can still be used here because i32 is a Copy type

    let s1 = gives_ownership(); // `s1` now owns the string
    println!("s1 = {}", s1);

    let s2 = String::from("hi");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    let len = calculate_length(&s1);
    println!("the length of '{}' is '{}'. ",s1,len);

    let s4 = add_suffix(s); // s is no longer valid
    // let s4 = String::from("hello ");
    // let s5 = add_suffix(s4);

    println!("{}", s4);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // `some_string` goes out of scope and gets dropped here
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    // `some_integer` goes out of scope but nothing special happens
}

fn gives_ownership() -> String {
    let some_string = String::from("hello!");
    some_string  // Returning the string moves ownership to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Ownership is returned to the caller
}

fn calculate_length(s:&String) -> usize {
    let length = s.len(); //len() returns the length of a string
    length
}

fn add_suffix(mut s:String) -> String {
    s.push_str("world!");
    s
}

//References are immutable by default

fn main2() {
    let s = String::from("hello");
    let s2;
    let b = false;
    if b {
        s2 = s;
    }
    // println!("{}", s); // When you assign s to s2 in the if block, the ownership of s is potentially moved to s2. However, because the condition b is false, s is not moved, and the assignment doesn't happen. But Rust still assumes that s2 might take ownership of s, making s invalid after the if block.

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
    //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;
}


