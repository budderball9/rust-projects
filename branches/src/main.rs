fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{number}");

    // loop goes forever or until it gets told to stop
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    let mut outer_loop = 0;
    let mut inner_loop = 0;

    'outer: loop{ 
        outer_loop += 1;
        loop {
            inner_loop += 1;
            
            if inner_loop == 5 {
                break 'outer;
            }
        }
    }

    println!("outer_loop: {outer_loop}");
    println!("inner_loop: {inner_loop}");

    // while
    let mut number = 0;
    while number != 5 {
        number += 1;
    }
    println!("{number}")
}