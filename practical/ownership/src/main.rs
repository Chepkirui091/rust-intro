fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
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
        let n: &String;
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

        println!("{name1}"); // will run but you can't do it in vice versa
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
}
