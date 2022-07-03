fn main() {
    let z = 400;
    println!("The value of the immutable variable is: {}", z);
    println!("Rust won't let me assign another value to this immutable variable:, saying 'cannot assign twice to immutable variable'.");

    let mut x = 5;
    println!("The value of the mutable variable is: {}", x);
    x = 6;
    println!("The value of the mutable variable after reassignment is: {}", x);

    const NUMBER_OF_SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3;
    println!("The number of seconds in three hours is: {}", NUMBER_OF_SECONDS_IN_THREE_HOURS);

    let y = 88;
    println!("The value of this immutable variable  is: {}", y);
    let y = y + 1;
    println!("The value of y after 'shadowing' it (defining it again using  'let y = y + 1') is: {}", y);

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y in the outer scope is: {}", y);

    let spaces = "     ";
    println!("spaces is '{}'", spaces);
    let spaces = spaces.len();
    println!("spaces is now an integer: {}. The type changed because I shadowed it. You can't reassign values of different types to a variable", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The meaning of life is {}", guess);
}
