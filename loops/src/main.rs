fn main() {

    //infinite_loop()

    nested_labelled_loop();

    loop_returns();

    while_loop();

    while_for_collections();

    for_loop_collections();

    another_for_loop();
}

fn infinite_loop() {

    // This is an infinite loop. You can stop this with ctrl+c 
    // use "break" and "continue" to handle the loops
    // break -> to break out of the loop
    // continue -> to skip the current iteration of the loop

    // IMP :- Both break and continue apply to the innermost loop(if you have loop inside loop).

    loop{
        println!("Hello, world!");
    }
}


fn nested_labelled_loop() {

    // You can label a loop like <'label_name>, and can break that specific loop using the labelname

    let mut count = 0;
    'outer_loop: loop {

        println!(" count = {}", count);

        let mut remainder = 10;
        loop {

            println!(" Remainder = {}", remainder);

            if remainder == 9 {
                break;
            }

            if count == 2 {
                break 'outer_loop;
            }

            remainder -= 1;
        }

        count += 1

    }
    println!(" End counter = {}", count);
}


fn loop_returns() {

    let mut count = 0;
    
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; // this will break the loop and also return the value
        }
    }; // Adding a ; here as this a statement afterall

    println!("Value of the result from loop_returns functions = {}", result);

}

fn while_loop() {

    println!("This is a while loop.");

    let mut x = 5;
    while x!=0 {
        println!("value = {}", x);
        x -= 1;

    }
    println!("Done with while loop!");
}

fn while_for_collections() {
    println!("This is while loop for collections(Arrays,tuples..)");

    let arr = [1,2,3,4,5];
    let mut index = 0;

    println!("The array is {:?}", arr); // syntax to print a whole array in Rust. use {:?}

    while index < 5 {
        println!("The value of arr at index {} is {}.", index, arr[index]);

        index += 1;
    }

    // This is a very unsafe way to write code and inefficient also. Use FOR loops instead in such situations.

}


fn for_loop_collections() {

    println!("This is a for loop.");

    let a = [1,2,3,4,5];
 
    // Syntax of FOR loop
    for element in a {
        println!("Value of item in array = {}", element);
    }

}

fn another_for_loop() {

    // using a for loop over a range of 1 to 5 (using 6 as upper bound is exclusive)
    // .rev() reverses the iterator
    for number in (1..6).rev() {
        println!("{number}!"); // another way to use format strings

    }
    println!("LIFTOFF!!!");
}
