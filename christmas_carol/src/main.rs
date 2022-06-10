fn main() {

    let days = ["first", "second", "third", "fourth", "fifth",
                 "sixth", "seventh", "eighth", "ninth", "tenth", 
                 "eleventh", "twelfth"];

    let quotes = ["A partridge in a pear tree",
                "two turtle doves, and",
                "Three french hens",
                "Four calling birds",
                "Five golden rings",
                "Six geese a-laying",
                "Seven swans a-swimming",
                "Eight maids a-milking",
                "Nine ladies dancing",
                "Ten lords a-leaping",
                "Eleven pipers piping",
                "Twelve drummers drumming"];


    println!("\nThe chrismas song lyrics ... \n");

    for i1 in 0..12 {
        println!("On the {} of Christmas, my true love sent to me", days[i1]);
        for i2 in (0..i1+1).rev() {
            println!("{}", quotes[i2]);
        }
        println!("\n");

    }
}


