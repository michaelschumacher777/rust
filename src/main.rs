fn main() {
    println!("Hello, world!");
    let x = 5;
    if x == 5 {
        println!("Test");
    }
    println! ("{}{}", "This is the result: ", test());

    let mut i = 0;
    while i <= 10 {
        i = i + 1;
        println! ("The value of i is {}", i);
    }

    for number in 0..4 {
        println!("{}", number);
    }
    println!("DONE!");
}

fn test() -> i32 {
    println!("Looking to see if this returns a value or not.");
    let y = 6;
    return y
}