use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;
    println!("In Floating points:{sum} {difference} {product} {quotient} {truncated} {remainder}");
    let t = true;

    let f: bool = false;
    println!("Booleans:{t} {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Character types: {c} {z} {heart_eyed_cat}");

    //invalid access of elements in an array
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

    println!("The value of the element at index {index} is: {element}");

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]);

    //Functions

    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");

}



fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
