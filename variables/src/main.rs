fn main() {
    // Variables are immutable by default.
    // In order to make it mutable, keyword 'mut' has to be used.
    // Compiler would fail on line 7 with error: cannot assign twice to immutable variable [E0384]
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants must annotate the type - u32 here
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // We are not allowed to mutate a variable's type.
    // Note: expected type `&str` found type `usize` [E0308]
    // Below code will not compile
    // let mut spaces2 = "   ";
    // spaces2 = spaces2.len();

    // If type annotation is not added, following error would occur:
    // Type annotations needed consider giving `guess` a type [E0282]
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar types: integer, floating-point numbers, Booleans, characters

    // Tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Array
    let arr = [1, 2, 3, 4, 5];

}
