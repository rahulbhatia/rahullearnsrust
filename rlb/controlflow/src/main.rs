fn main() {
    println!("Hello, world!");
    println!("Value: {}", if_statments(9));
    println!("Value: {}", if_statments(10));
    println!("Value: {}", if_statments(11));

    println!("Inline If {}", inline_if(10));

    unconditional_loop();

    println!("Returning from loop!? {}", returning_from_loop());

    for_loop();
}

fn if_statments(x: i32) -> i32 {
    if x < 10 {
        x/2
    }
    else if x == 10 {
        x
    }
    else {
        x * 2
    }
}

fn inline_if(x : i32) -> i32 {
    let number = if x < 10 {
        x/2
    }
    else {
        x*2
    };
    number
}

fn unconditional_loop() {
    let mut count = 0;
    loop {
        println!("Again!");
        count = count + 1;

        if count > 10 {
            break;
        }
    }
}

fn returning_from_loop() -> i32{
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    result
}

fn for_loop() {
    for number in 0..4 {
        println!("Number is {}", number);
    }

    let x : [i32; 5] = [10, 20, 30, 40, 50];
    for number in x.iter() {
        println!("Number is {}", number);
    }
}
