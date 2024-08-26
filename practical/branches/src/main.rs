//If expressions( must be bool for it to work)

fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust only executes the block for the first true condition
    //Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code.
    {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        }else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        }  else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    //Using if in a let Statement
    {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }
    // {
    //     let condition = true;
    //
    //     let number = if condition { 5 } else { "six" };
    //
    //     println!("The value of number is: {number}");
    // }

    // {
    //     let x = if cond { 1 } else { 2 };
    // }
    // Repetition with loops

    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

   // Loop Labels to Disambiguate Between Multiple Loops you do that by using break or continue, and you start with a single quote
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }


}


