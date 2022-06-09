use std::io;

fn fahrenheit_to_celcius(val: f32) -> f32 {
    let val = (val - 32.0) * (5.0/9.0);
    return val
}

fn main() {

    println!("Please enter the temperature in Fahrenheit..");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: f32 = input.trim().parse().expect("Please enter a valid number.");

    println!("You entered {input}");


    let input = fahrenheit_to_celcius(input);

    println!("The entered temperature in celcius is = {input:.2} degrees");


}
