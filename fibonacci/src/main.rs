use std::io;
// 0 1 1 2 3 5 8 13 21
fn main() {

    let number_to_generate: u32 = loop {
        println!("Please enter the number of fibonacci numbers you'd like to generate:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue
        };
    };

    let mut fibonacci_numbers: Vec<u32> = Vec::new();

    println!("Outputting the fibonacci numbers up to {number_to_generate}:");

    for number in 0..number_to_generate {
        if number == 0 {
            fibonacci_numbers.push(number);
            continue;
        }
        else if number == 1 {
            fibonacci_numbers.push(number);
            continue;
        }

        let val: u32 = {
            &fibonacci_numbers[(number - 1) as usize] + &fibonacci_numbers[(number - 2) as usize]
        };

        fibonacci_numbers.push(val);
    }

    for number in fibonacci_numbers {
        print!("{number}, ");
    }
}
