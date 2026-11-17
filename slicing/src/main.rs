fn main() {
    let sentence = String::from(" meowmeo w");
    let first_word = first_word(&sentence);
    println!("{first_word}")
}

// takes a string of words seperated by spaces and returns the first word

// returns the index of the first space in the string
fn first_word(s: &String) -> usize{

    // convert string to array of bytes
    let bytes = s.as_bytes();

    // iterator
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}