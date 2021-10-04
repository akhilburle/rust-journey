#![allow(unused_variables, dead_code)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let email = String::from("Newemail@email.com");

    let mut user2 = User {
        email,
        username: String::from("someusername123"),
        ..user1 // Takes the rest of the struct's variables from user1
    };

    user2.email = String::from("anotheremail@example.com");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    use std::fs::File;
    use std::io::ErrorKind;
    {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

}
