use std::io;

fn main() {

    loop{
        println!("Hello, please enter a number for generate nth fibonacci number");

        let mut number = String::new();

        io::stdin().read_line(&mut number).expect("Failed to read line.");

        let number: u32 = number.trim().parse().expect("Failed, Please only enter valid numbers");

        let result = generate_fibonacci(number);

        println!("The n'th fibonacci number is {result}");

    }
    

}


fn generate_fibonacci(n: u32) -> u32 {

    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else {
        let mut n_minus_2 = 0;
        let mut n_minus_1 = 1;
        let mut current = n_minus_2 + n_minus_1;

        for _ in 2..n+1 {
            current = n_minus_2 + n_minus_1;
            n_minus_2 = n_minus_1;
            n_minus_1 = current;

        }
        return current;

    }
}
