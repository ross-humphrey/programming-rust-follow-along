use std::str::FromStr; // Brings the standard library trait FromStr into scope
use std::env; // Imports the env module

/*
What is a trait?
- A trait is a collection of methods that types can implement.
- Any type that implements teh FromStr trait has a from_str method that
tries to parse a value of that type from a string.
- A trait must be in scope to use its methods.
*/

fn main() {
    let mut numbers = Vec::new(); // Vec is rusts growable vector type (similar to Pythons list)
    // The numbers variable even though a vector is designed to have values pushed on to it - must be marked as 'mut'

    for arg in env::args().skip(1) { // First argument is the name of the program run - so we want to skip that omitting the first value
        numbers.push(u64::from_str(&arg).expect("error parsing argument")) //from_str does not return a u64 - but instead a result value, pass or fail
    }
    // IMPORTANT : Rust does not have exceptions - all errors are handled using Result or panic (more to come on this)

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ..."); //eprintln will print to the standard error stream
        std::process::exit(1);
    }

    let mut d = numbers[0];
    /*
    When we iterate we want to tell Rust that ownership of the vector should remain
    with numbers; we are merely borrowing its elements for the loop. The & operator in
    &numbers[1..] borrows a reference to the vectors elements from the second onward.

    The for loop iterates over the referenced elements, letting m borrow each element in
    succession. The * operator in *m dereferences m, yielding the value it refers to.

    Since numbers owns the vector - Rust then automatically frees it when numbers goes
    out of scope at the end of main

    To summarize - &x borrows a reference to x and *r is the value that the reference r refers to
     */
    for m in &numbers[1..]{
        d = gcd(d, *m)
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);

}

// u64 - unsigned integer
// mut - stands for immutable so the function body can assign to them
fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Rust machine integer types names reflect their size and signedness - i32 is signed 32, u8 is unsigned 8 bit
    // ! << is a call to a macro invocation. If this is not true, then the program is terminating, called a 'panic'
    assert!(n != 0 && m != 0); // Assertions are always checked in rust
    // By default once variable is initialized it is immutable
    while m != 0 {
        if m < n {
            // let - declares a local variable
            let t = m;
            m = n;
            n =t;
        }
        m = m % n // Modulo operator - sets remainer to 0 (shows divisor). And returns
    }
    n
}
/* Rust has a return statement, but the gcd function does not need one. If a fiunction
body ends with an expression that is not followed by a semicolon thats the return value

Any block surrounded by curly braces can function as an expression. Such as:
{
    println("evaluating cos x");
    x.cos()
}

This is typical in Rust to use this form to establish the functions value when control
"falls off the end" of the function - use return statements only for explicit early returns from
the midst of the function.
 */

#[test] // Marks as a test function, to be skipped in normal compilations but included and called if we run using cargo test
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
/*
In the above code the #[test] marker is an example of an attribute.
'attributes' are an open-ended system for marking functions and other declarations with
extra information. They are used to control compiler warnings, code style checks and include
code conditionally - this tells Rust how to interact with code.
*/

// A Rust package whether a library or an executable is called a crate. Cargo and crates.io both derive their names from this term.
