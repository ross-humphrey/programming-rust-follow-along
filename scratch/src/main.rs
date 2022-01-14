// Arrays example

fn main() {
    // Defining arrays - [type; capacity]
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropeda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

// Common case of long array filled with some value, write [V; N], where V is the value each element has, N is the length
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // If you need an array whose length varies at run time - use vector

    // A vector Vec<T> is a resizable array of elements of type T, allocated on the heap
    let mut primes = vec![2, 3 ,5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    // Can build a vector by repeating a given value a certain number of times - using syntax that imitates array literals
    // The vec! macros is equivalent to calling Vec::new to create a new empty vector and pushing the elements onto it
    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step","on","no","pets"]);

    //build a vector from the values produced by an iterator
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0,1,2,3,4]);

    // As with arrays - you can use slice methods on vectors
    // A palindrome
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    // capacity method on a vector returns the number of elements it can hold without reallocating
    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(),0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    // Typically prints "capacity is now at 4":
    println!("capacity is now {}", v.capacity());

    /*
    Insert and remove elements wherever you like in a vecotr,
    operations shift all elements after affected position forward
    or backward - slow if the vector is long
     */
    let mut v = vec![10, 20 , 30 , 40, 50 ];

    // Make element at index 3 be 35.
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35 ,40, 50 ]);

    //Remove element at index 1
    v.remove(1);
    assert_eq!(v, [10,30,35,40,50]);

    // Pop can be used to remove the last element and return it
    let mut v = vec!["Snow Puff", "Glass Gem"];
    assert_eq!(v.pop(), Some("Glass Gem"));
    assert_eq!(v.pop(), Some("Snow Puff"));
    assert_eq!(v.pop(), None);

    // a for loop can be used to iterate over a vector
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
        if l.len() % 2 == 0 {
            "functional"
        } else{
            "imperative"
        });
    }

    // Slices
    /*
    A slice - written [T] without specifying the length - is a region of
    an array or vector. Since it can be any length - this cannot be stored in a variable
    and instead it is always passed by reference.

    A reference to a slice is a 'fat pointer' a two word value comprising a pointer to the
    slices first value and the number of elements in the slice.
     */

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v; // Converts the &Vec<f64> reference and &[f64; 4] reference to slice references that point directly to data.
    let sa: &[f64] = &a;

    fn print(n: &[f64]){ // Takes a slice argument as reference, because it takes a reference to a slice it can apply to vectors or arrays.
        // Many methods for vectors or arrays are methods defined on slices. Like - sort or reverse = methods on slice type [T]
        for elt in n {
            println!("{}", elt);
        }
    }
    print(&a);
    print(&v);

    // Can get a reference to a slice of an array or vector, or a slice of an existing slice by indexing it with a range
    println!("First two elements of v");
    print(&v[0..2]); // Prints the first two elements of v
    println!("All elements starting with a[2]");
    print(&a[2..]);//Prints elements of a starting with a[2]
    println!("Print v[1] and v[2]");
    print(&sv[1..3]);

    /*
    Slices almost always appear behind references, so we often refer to types ike &[T] or &str as "slices"
    using shorter name for the more common concept.
     */

    // String literals
    let speech = "\"Ouch!\" said the well.\n";// Single quotes dont need a backslash escape and double quotes do
    // A string can span multiple lines
    println!("In the room the women come and go,
       singing of Mount Abora");

    // With a backslash at the end of the line - the newline character is ignore as well as any white space
    println!("In the room the women come and go,\
    singing of Mount Abora");

    // A raw string is tagged with lowercase r. All backslashes and whitespace characters inside a raw string are included in the string.
    let default_win_install_path = r"C:\Program Files\Gorillas";


}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}



