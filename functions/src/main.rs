fn main() {

    // another_function(5, 'h');

    // the {} block evaluates to 4 
    // y gets assigned to 4
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {y}");

    let x = five();
    println!("x: {x}")

}

fn another_function(x: i32, unit_label: char) {

    println!("measurement: {x}{unit_label}");

}

fn five() -> i32 {
    5
}