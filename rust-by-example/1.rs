// Just like C++, comments start with '//'
/*
    Comments can also be C-style like this
*/

/// Comments for generating library docs
fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");
    
    let x = 3 * 3 * 3;
    println!("\nMy favorite number is {x}", x=x);

    demonstrate_debug_formatting();
    demonstrate_display_functionality();
    demonstrate_sequential_display();
}

// Can compile code in rust using rustc
// $ rustc 1.rs

// format!() is just like print!() but it doesn't output it to std::out
// eprint!() is just like print!() but outputs to std:error instead
// appending with ln like println!() or eprintln!() is the same but
// appends to std::out a new line

// You can output non std types for debugging purposes by deriving
// the fmt::Debug implementation. "Deriving" means it is automatically
// created


/* Copied from the book  v v v v v v */

// This structure cannot be printed either with `fmt::Display` or
// with `fmt::Debug`.
#[allow(dead_code)]
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`. 
//This is not true for fmt::Display which must be manually implemented.
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn demonstrate_debug_formatting() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.\n",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!\n", DebugPrintable(3));


    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Can pretty print the debug with :#?    
    // Without pretty print
    println!("{:?}", peter);
    // Pretty print
    println!("{:#?}", peter);

    // You can also implement how to display the type
    // if you don't like fmt::Debug, by implementing
    // fmt::Display
}

#[derive(Debug)]
struct Complex(f64, f64);

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // No semicolon ensures that the return value is not suppressed
        // and is instead piped as the return value
        write!(f, "{} + {}i", self.0, self.1)
    }
}

fn demonstrate_display_functionality() {
    println!("{}", Complex(3.4, 6.7));
}

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn demonstrate_sequential_display() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}