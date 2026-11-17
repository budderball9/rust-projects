fn main() {
    let f_temp = 78;
    let c_temp = convert_fahrenheit_to_celsius(f_temp);
    println!("temperature in fahrenheit: {f_temp}");
    println!("temperature in celsius: {c_temp}");

    let c_temp = 35;
    let f_temp = convert_celsius_to_fahrenheit(c_temp);
    println!("temperature in celsius: {c_temp}");
    println!("temperature in fahrenheit: {f_temp}");


    let fib = fibonacci_number(50);
    println!("fib number: {fib}");
}

fn convert_fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) / (9 / 5)
}

fn convert_celsius_to_fahrenheit(temperature: i32) -> i32 {
    temperature * (9 / 5) + 32
}

fn fibonacci_number(num: i32) -> i32 {
    
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }

    return fibonacci_number(num - 1) + fibonacci_number(num - 2)
}