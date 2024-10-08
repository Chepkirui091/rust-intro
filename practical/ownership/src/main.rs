//----------- Ownership rules -----
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    { // s is not valid here, it's not yet declared
        let s = String:: from("hello"); // s is valid from this point onwards
        //do stuff with s
    } // this scope is now over, and s is no longer valid



    let x = true;
    read(x);
    {
        let n = 5;
        let y = plus_one(n);
        println!("The value of y is: {y}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    {
        let n;
        {
            let name = String::from("Caleb");
            n = &name;
            println!("{n}")
        }
    }
    //References
    {
        let score =50;
        let points = score +3;
        println!("{score} {points}")
    }

    //strings just have clone and not copy
    {
       let name = String::from("Caleb");
        let name2 = name.clone(); //cloning
        let name3 =&name; //referencing where you use &

        println!("{name} {name2} {name3}")
    }

    //dereference
    {
        let mut name = String::from("Daphne");
        let name1 = &mut name; //you must use mut and can't reference more than once
        *name1 = String::from("Daph");

        println!("{name1}"); // will run, but you can't do it in vice versa
        println!("{name}");
    }

    fn greet(first:String,last:String) -> (String, String){
        println!("Hello {first} {last}");
        (first,last)
    }
     {
        let mut first = String::from("Chepkirui");
        let mut last = String::from("Chep");
        (first, last) = greet(first,last);
        println!("Hello {first} {last}")
    }

    // variables live in the stack
    {
        let n = 5;
        let y = plus_two(n);
        println!("The value of y is: {y}");
    }

    fn plus_two(x: i32) -> i32 {
        x + 1
    }

    //Boxes live in the heap
    // However copying data take up a lot of memory e.g program that copies an array with 1 million elements:
    {
        let a = [1_000_000];
        let b = a;
        println!("{:?}",b)
    }

    //pointer
    // {
    //     let b = Box::new([0; 100]);
    //     free(b);
    //     assert!(b[0] == 0);
    // }
    //
    //
    // {
    //     let a_num = 4;
    //     make_and_drop();
    // }
    // fn make_and_drop() {
    //     let a_box = Box::new(5);
    // }
    {
        let x: &str = "hello";
        let y = 22;
        b(x);
    }

    fn b(x: &str) {
        println!("{x}");
    }
}



