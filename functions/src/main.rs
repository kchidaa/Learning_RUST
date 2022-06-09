/*
This is a multiline comment in rust, maybe just like in c.

In RUST everything starts at the main function. 

You can add functions before or after the MAIN function and it doesn't matter.
*/


// this is a single line comment
// this is another single line comment
// you can add comments in a line after statement or expressions like in line 42

// expression do not end with ; 
// Only statements end with ;
// expression returns some values while statements do not
// line 32 is an statement, while line 42 is an expression


fn main() {
    println!("Hello, world!");

    another_function(55, 'c');
}

fn another_function(x: u32, unit_label: char) {
    println!("Hello world from another function");

    println!("Main function passed {} and {} to this function!", x, unit_label);

    // statement
    let y = get_something(5);

    println!("The get_five function returned the value {}", y);
}


fn get_something(x : u32) -> u32 {
    
    x + 1  //expression
    // adding a ; will results in compile error
}

