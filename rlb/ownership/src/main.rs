fn main() {
    println!("Hello, world!");

    // Ownership of variables follows the move principle by default.

    let x = 10; // x is a stack variable, stores a i32 by default

    let name = String::from("Rahul");
    // this constructs a string on the heap. 
    // Stack has a reference + a size + length

    call_with_copy(x);
    call_with_move(name);

    // Now name is an invalidated reference, so we cannot use it.
    // This will fail: println!("Invalidated string {}", name);

    let name = String::from("Rohan");
    // call our function by loaning the string immutably
    call_with_reference(&name);
    println!("Name is : {}", name);

    // can only have one mutable reference at a time (single writer)
    // or many immutable references (readers)
    let mut name = String::from("John");
    mutate_with_reference(&mut name);
    println!("name is now : {}", name);

    println!("substring of name is  : {}", return_substring(&name[..]));
}

fn call_with_copy(val : i32) {
    println!("Copied a number: {}", val);
}

fn call_with_move(input : String) {
    println!("The input was {}", input);
}

//This function takes an (immutable) reference to a string. 
// It's referred to as "borrowing"
fn call_with_reference(input : & String) {
    println!("The input was {}", input);
}

// takes a mutable reference

fn mutate_with_reference(input : &mut String) {
    input.push_str(" | added this");
}

fn return_substring(input : & str) -> &str {
    &input[3..]
}