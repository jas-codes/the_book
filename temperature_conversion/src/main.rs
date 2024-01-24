use std::io;

fn main() {
    loop {
        println!("Please input your temperature unit, C or F?");

        let mut input_str = String::new();

        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line.");

        let unit: char = match input_str.trim().parse()
        {
            Ok(val) => val,
            Err(_) => continue
        };

        let mut input_num = String::new();

        println!("Please input your temperature.");
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line.");

        let temperature: u32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if unit.eq_ignore_ascii_case(&'c') {
            let temperature_in_f = (temperature * 9 / 5) + 32;
            println!("That temperature converted to celsius is {temperature_in_f}")
        }
        else if unit.eq_ignore_ascii_case(&'f') {
            let temperature_in_c:i32 = (temperature as i32 / 5 / 9) - 32;
            println!("That temperature converted to celsius is {temperature_in_c}")
        }
        else {
            println!("Invalid temperature unit, try again.");
            continue
        }

        break;
    }
}
