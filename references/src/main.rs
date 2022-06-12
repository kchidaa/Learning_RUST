/*

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // variable s1 loses its ownership in this scope

    println!("The length of '{}' is {}.", s2, len); 
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

*/

// Instead of the above method, using reference we can withhold the ownership of variable in our scope


fn main() {
    let s = String::from("Hello");

    let len = calculate_length(&s);

    println!("Length of the string {s} is {len}");

    mutable_reference();

    mutiple_mutable();

    immutable_reference_scope();


}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

//   When functions have references as parameters instead of the actual values, 
//   we won’t need to return the values in order to give back ownership, because we never had ownership.

// Just as variables are immutable by default, so are references. 
// We’re not allowed to modify something we have a reference to. check below. 


// fn change(some_string: &String) {
//     some_string.push_str(", world"); // This fails and won't compile
// }

// To solve this, we have declare the variable as mutable as follows..

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// NOTE:- Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time. 

/*

fn mutiple_mutable() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", &r1);

    let r2 = &mut s;        // Compile error here

    println!("{}, {}", r1, r2);



}

This is not a bug, but a feature that RUST has in order to solve "Data race" issue at compile time

A data race is similar to a race condition and happens when these three behaviors occur:

-> Two or more pointers access the same data at the same time.
-> At least one of the pointers is being used to write to the data.
-> There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; 
Rust prevents this problem by refusing to compile code with data races!


As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
see below how.
 */

fn mutiple_mutable() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

        println!("{}", &r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("{}", r2);


}

// Rust is okay as long as you use immutable references even multiple times
// But it doesn't allow more than one mutable references
// We also cannot have a mutable reference while we have an immutable one to the same value. 

// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

/*

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;                    // no problem
    let r2 = &s;                    // no problem
    let r3 = &mut s;                // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}


*/

// Good thing is you can create an mutable reference once the scope of a immutable reference is over
// Btw the scope of an immutable reference gets over after the last time that reference is used

// For instance, this code will compile because the last usage of the immutable references, 
// the println!, occurs before the mutable reference is introduced

fn immutable_reference_scope() {

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


}

/*

dangling pointer--> a pointer that references a location in memory 
that may have been given to someone else--by freeing some memory while preserving a pointer to that memory

 In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, 
 the compiler will ensure that the data will not go out of scope before the reference to the data does. see below 

 This will not compile..

 fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. 
But we tried to return a reference to it. 
That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.


Solution to this is..

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // This works without any problems. Ownership is moved out, and nothing is deallocated.

*/