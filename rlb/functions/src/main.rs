fn main() {
    println!("Hello, world!");
    another_function();
    new_fun_with_params(12, 1254);
    println!("10^2 is {}", square(10));
}


fn another_function() {
    println!("Another function printed");
}

// Functions need to declare the types of their inputs
fn new_fun_with_params(x: i32, y: i32) {
    println!("X is {}", x);
    println!("Y is {}", y);
}

fn square(x: i32) -> i32 {
    x*x
}