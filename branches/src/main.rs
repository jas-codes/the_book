fn main() {
    let x = 3;

    if x < 5 {
        println!("Condition was true.");
    }
    else {
        println!("Condition was false.");
    }

    if x != 0 {
        println!("x is a non-zero value.");
    }

    multiple_conditions(6);
    if_in_a_let();
}

fn multiple_conditions(x: i32) {
    if x % 4 == 0 {
        println!("Number must be divisible by 4.");
    }
    else if x % 3 == 0 {
        println!("Number is divisible by 3.");
    }
    else {
        println!("Number is not divisible by 3 or 4.");
    }
}

fn if_in_a_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}