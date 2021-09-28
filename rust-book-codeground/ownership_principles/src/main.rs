fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    
    // This line wouldn't work because the string in s has already been moved
    // println!("{}", s); 

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // By using a reference to s1, we don't have to follow convoluted mechanisms to pass a non-primitive type

    println!("The length of '{}' is {}.", s1, len);

    mutable_references();
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

/// The problem with normal references is that the called function
/// cannot modify the referenced object because it does not own it.
/// It is only borrowing it for the duration of the function execution
/// 
/// In a way, it is like `const&` in C++
/// 
/// If we want to be able to modify the variable in the called function,
/// we need to pass a mutable reference to the called function. This
/// function demonstrates that concept
fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("The string is {}", s);

    /*
    // This won't work because you cannot pass a mutable reference of
    // an immutable variable
    let s = String::from("hello");

    change(&mut s);
    println!("The string is {}", s);
    */

    // You can also pass an immutable reference of a mutable variable
    // This is just for demonstration, doesn't serve any purpose in this
    // example
    let mut s = String::from("hello");
    println!("The length of the string is {}", calculate_length(&s));
}

fn change(str : &mut String) {
    str.push_str(", world!");
}