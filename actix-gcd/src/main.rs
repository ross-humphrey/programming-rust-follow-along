use actix_web::{web, App, HttpResponse, HttpServer}; // Each name in curly braces becomes available to use


// Define a rust structure type that represents the values we expect from our form
/* Tells serde crate to examine the type on compile and generate
code to parse a value of this type from data in the format that HTML forms use
for post requests.

This annotation will let you parse a GcdParameters value from almost any type of structured data
like JSON, TOML, YAML etc  - a 'Serialize' attribute will also allow for the reverse - when you can
take Rust values and write them out to any structured format.
*/

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    /*
    r# - rusts "raw string" syntax. Any character can appear in a raw string without being escaped
     */
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

use serde::Deserialize;
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} \
                 is <b>{}</b>\n",
                form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

/*
What is a closure?
- A closure is a value that can be called as if it were a function.
A closure can take arguments - these arguments would appear between the || vertical bars.
{...} is the body of the closure.
 */

/*
What happens when we start our server?
- Actix starts a pool of threads to handle incoming requests. Each thread calls our closure to get
a fresh copy of the App value that tells it how to route and handle requests.
The closure calls App::new to create a new empty App and then calls its route method ot add a single
route to the path '/' - a handler is also included on that route to handle get requests. App is the
closures return value.
 */


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
