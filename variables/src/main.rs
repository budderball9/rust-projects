fn main() {

    // shadowing
    // declare x
    let x = 5;
    println!("{x}");

    // new x declared which "shadows" the previous one
    let x = x + 1;

    {
        // creates copy of x that deletes 
        // when it goes out of scope
        let x = x * 2;
        println!("inner scope of x: {x}");
    }

    println!("{x}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y in tup is: {y}");

    // variable.{index}
    // to select a specific index in a tuple
    // _variable to show its unused
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // arrays
    let a = [1, 2, 3, 4, 5];

    // array access
    // 1
    let _first = a[0];
    // 2
    let _second = a[1]
    
    // array length 5 full of 32 bit integers
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // same as writing:
    // let a = [3, 3, 3, 3, 3];
    let _a = [3; 5];

    
}
