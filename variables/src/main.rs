
fn main() {
    
    mut_example();
    shadowing_example();    
}

fn shadowing_example() {
    // shadowing allows us to re-bind a variable to a value
    let y = 10;
    println!("Y: {}", y);
    let y = "hello";
    println!("Y: {}", y);
}

fn mut_example() {
    // this needs to be tagged mut so that we can change it later.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}