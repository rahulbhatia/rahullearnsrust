fn main() {

    // i32 is typically the default
    let x : i32 = 10;
    let y : i16 = 10_000;
    let z : i8 = -15;

    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);

    // f64 is typically the default
    let k : f64 = 127_000.000_12;
    println!("k: {}", k);

    // note we can only mod matching types
    let x2: i32 = 17;
    let remainder = x2 % x;
    println!("remainder: {}", remainder);

    // booleans are pretty normal. They're only 1 byte in size

    let value : bool = true;
    let opp : bool = !value;
    println!("opp: {}", opp);

    // characters in rust are unicode so you have the ability to do shit like this:
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Cat heart: {}, {}", heart_eyed_cat, z);

    println!("There are also compound types, which we'll explore next");

    tuple_example();

    array_example();
}

fn tuple_example() {
    // create a tuple
    let tup = (500, 6.4, 1);
    // disassemble a tuple
    // note that the underscore means we're not going to use the value so don't warn us.
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    // index into a tuple
    let y = tup.1;
    println!("The value of y is: {}", y);

}

fn array_example() {
    let arr1 : [i32; 5] = [1,2,3,4,5];
    println!("index 0 is: {}", arr1[0]);
    
    // This is invalid code
    // let index = 10;
    // arr1[index];

}
