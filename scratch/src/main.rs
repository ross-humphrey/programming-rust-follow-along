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
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

