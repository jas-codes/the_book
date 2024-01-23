fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    nested_and_labeled_loops();
    conditional_loops_with_while();
    while_loop_over_array();
    for_loop_over_array();
    for_loop_countdown();
}

fn nested_and_labeled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn conditional_loops_with_while() { //for loop with a range is preferred here as its safer
    println!("While loop:");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn while_loop_over_array() {
    println!("While loop:");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() { //error prone so use the for approach instead
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop_over_array() {
    println!("For loop:");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

fn for_loop_countdown() {
    println!("For loop:");

    for element in (1..4).rev() {
        println!("{element}!");
    }

    println!("LIFTOFF!");
}