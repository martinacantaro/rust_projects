fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const NUMBER_OF_SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3;
    println!("The number of seconds in three hours is: {}", NUMBER_OF_SECONDS_IN_THREE_HOURS);

    let y = 88;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y in the outer scope is: {}", y);

    let spaces = "     ";
    println!("spaces is {}", spaces);
    let spaces = spaces.len();
    println!("spaces is {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The meaning of life is {}", guess);
}
