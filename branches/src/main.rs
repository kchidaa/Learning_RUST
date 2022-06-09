fn main() {
    let x = 6;

    // 'if' is an expression (don't use ;)
    // The value of conditions must return a BOOL, else it will fail.
    if x % 7 == 0 { 
        /* These block of code associated with the Conditional branching
         like IF, MATCH are called ARMS. */
        println!("The number is divisible by 7!");
    } else if x % 2 == 0 { // Here also the condition must return a BOOL
        // This is another arm
        println!("The number is divisible by 2!");
    } else if x % 3 == 0 {
        // This is another arm
        println!("The number is divisible by 3!");
    } else {
        // This is another arm
        println!("The number is not divisible by 2,3 or 7!")
    }

    // The below condition will fail and not compile unlike in python. Rust only accepts BOOL.
    // if x {
    //     println!("This is not correct and will fail!");
    // }

    // Calling another function
    using_if_inside_let()

}

fn using_if_inside_let() {

    let condition = true;
    let number = if condition {5} else {0}; 
    // This is a statement as we assigning value to number using let                                           
    // Note that ; is not used in inner blocks of if expression

    println!("The value of using_if_inside_let function is {}", number);
}


// A block of code evaluates to the last expression in the block and return the value from it.
// Simple numbers are also considered expersssion if ; is not used.


/* Arms of a branch like 'IF' should return value of same type, or else it will fail.
The below expression will not work as it has diffrent type of return values.

 let number = if condition {5} else {"six"}; 

The above code fails because, the type of variable "number" is not known at compile time 
and it is calculated at Runtime. RUST does not allow this. */