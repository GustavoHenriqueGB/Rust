// Definition of an costant:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
/*  Definition of an immutable variable
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // this code returns an error: "cannot assign twice to immutable variable `x`"
*/
// Definition of an mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

// Shadowing, 
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");

    let spaces = "   ";
    let spaces = spaces.len(); // changing the type of a variable by sahdowing
    println!("Number of spaces: {spaces}");
}