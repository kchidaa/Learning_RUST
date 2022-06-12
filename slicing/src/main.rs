/*
Here’s a small programming problem: write a function that takes a string and 
returns the first word it finds in that string. If the function doesn’t find a space in the string, 
the whole string must be one word, so the entire string should be returned.
*/



fn main() {
    let s = String::from("This is a test string!");

    let index : usize = first_word(&s);         // index will get the value 4

    println!("The first word is = {}", &s[..index]);  // Syntac to slice a container

    //s.clear()                           // this empties the String, making it equal to ""
                                          // this is an error, but it still compiles which is unexpected
                                          // hence use string slice to avoid this 

    // let len = s.len();

    // &s[0..4] = This
    // &s[..4]  = This
    // &s[5..9] = is a
    // &s[5..] = is a test string
    // &s[5..len] = is a test string
    // &s[0..len] = This is a test string
    // &s[..] = This is a test string


    let word = another_first_word(&s);

    //s.clear()                         // Adding this line will result in compile error 
                                        // as there is not string, which is expected behavior

    println!("The first word is = {}", word);



    // Let's see string literals

    let _ls = "Hello, world!";      // Add _ for unused variables
    
    // The type of ls here is &str: it’s a slice pointing to that specific point of the binary. 
    // This is also why string literals are immutable; &str is an immutable reference.



    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = yet_another_first_word(&my_string[0..6]);
    let _word = yet_another_first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = yet_another_first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = yet_another_first_word(&my_string_literal[0..6]);
    let _word = yet_another_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = yet_another_first_word(my_string_literal);



    // Slicing also works for other containers like arrays and tuples

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); 
    

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}


fn another_first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]              // Directly return the string slice
        }
    }
    &s[..]                              // Directly return the string slice

}

// A more experienced Rustacean would write the signature below instead because 
// it allows us to use the same function on both &String values and &str values.


fn yet_another_first_word(s: &str) -> &str {        // Main change here

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]              
        }
    }
    &s[..]                              

}