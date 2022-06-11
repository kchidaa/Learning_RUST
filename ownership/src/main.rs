
// In rust garbage collection(memory deallocation occurs when a variable goes of out scope)
// below, both x and y go out of scope after } and the memory allocated to them will be freed. 
// same for z, _m and _n


fn main() {
    let x = 5; // stored in stach
    let y = x; // stored in stack

    println!("This should work {}, {}", x, y); // works

    let a = "hello";
    let b = a;


    println!("This should work {}, {}", a, b); // works


    let z = String::from("hello"); // stored in heap
    let _m = String::new(); // stored in heap

    let _n = z;

    // println!("this is result in error as z is invalid now {}", z); // Error


    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // This works


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let p = 5;                      // x comes into scope

    makes_copy(p);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    
    let x1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let x2 = String::from("hello");     // s2 comes into scope

    let x3 = takes_and_gives_back(s2);  // s2 is moved into
                                    // takes_and_gives_back, which also
                                    // moves its return value into s3

} // Here, p goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Here, x3 goes out of scope and is dropped. x2 was moved, so nothing
  // happens. x1 goes out of scope and is dropped.




fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}

/*

In line 8 and 9, both x and _y are pushed to stack as we know the size of both in compile time,
and it is obviously faster to access in stack. Hence once out of scope both will be deleted


Variable whose size is not known at compile time is allocated in heap and in rust, 
deep copy is not done automatically, instead shallow copy(move) is done for heap memory.

x = "hello" :- this is a string literal and is immutable and hence stored in stack and size is known as compile time
x = String::from("hello") :- this is special type called String. It is mutable and dynamic in size so size in not
                                known at compile time. and hence stored in heap.

As String (not string literals, line 20 and 21) are stored in heaps, the pointer to its location is passed
in to stack. when we copy it, we only copy its pointer to new variable to another variable in stack. hence when both variables
goes out of scope, it results in double free error (trying to release the same memory twice). which 
results in security vulnerabilities. This is a memory safety bug.

To solve this rust invalidates z once it is copied(moved) to another variable.

In line 20 and 23. once z is copied to _n. z is invalidated to avoid double free error.

hence, when _n goes out of scope it is then dropped(memory deallocated) and we don't need to worry about z.


It you want to deep copy data in heap, you need to clone it. which creates another copy in heap.
for example like in line 28 and 29. here s1 is not invalidated. using clone makes sense only when heap 
memory is involved as any copies on stack memory are deep copies(clones). 


The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
When a variable that includes data on the heap goes out of scope, 
the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

While this works, taking ownership and then returning ownership with every function is a bit tedious. 
What if we want to let a function use a value but not take ownership? 
It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, 
in addition to any data resulting from the body of the function that we might want to return as well.

But this is too much ceremony and a lot of work for a concept that should be common. 
Luckily for us, Rust has a feature for using a value without transferring ownership, called references.


*/
