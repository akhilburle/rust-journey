fn main() {
    another_function(5, 6);

    let x = 7;
    let y = {
        let x = 8;
        x+1
    };
    println!("x is {} and y is {}", plus_three(x), y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_three(x : u32) -> u32 {
    x + 3
}