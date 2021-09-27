use std::fmt;

#[allow(unused_variables, unused_assignments)]
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12f64; // Type i64 is inferred from another line
    inferred_type = 4294967296f64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    println!("{}", mutable);
    // // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // // Variables can be overwritten with shadowing.
    // let mutable = true;

    matrix_activity();
    println!("\n{}", transpose(Matrix(1.1, 1.2, 2.1, 2.2)));
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

struct Matrix(f32, f32, f32, f32);
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn matrix_activity() {
    println!("{}", Matrix(1.1, 1.2, 2.1, 2.2))
}