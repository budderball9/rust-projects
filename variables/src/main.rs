fn main() {

    let x = 5;
    println!("{x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("inner scope of x: {x}");
    }

    println!("{x}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y in tup is: {y}");

}