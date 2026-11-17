fn main() {
    
    let s1 = String::from("hello");

    // we are passing a reference to s1 into the function
    // &s1 refers to the value of s1 but does not own it (borrowing)
    let len = calculate_length(&s1);

    println!("the length of {s1} is {len}");


    // mutable references:
    // unless we specify, references will be immutable
    let mut s = String::from("hello");
    println!("mut s before change function: {s}");
    change(&mut s);
    println!("mut s after change functin: {s}");


    // //cannot have 2 mutable references
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}"); // will not compile

    // // cannot have mutable reference after immutable reference
    // let mut s = String::from("hello");
    // let r1 = &s; 
    // let r2 = &s; 
    // let r3 = &mut s; 
    // println!("{r1}, {r2}, and {r3}");   // will not compile
    // // immutable references cant have the value change suddenly


}


// usize: pointer sized unsigned integer
fn calculate_length(s: &String) -> usize { 
    s.len()
}
// since s is just a reference, what its refering to
// does not go out of scope

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}