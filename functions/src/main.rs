fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_a_parameter(5);
    print_labeled_measurement(9, 'j');
    expression();
    let x = function_with_return_value();
    println!("The value of the return function is: {x}");
    let y = plus_one(5);
    println!("The value of the plus one function is: {y}");
}

fn another_function() {
    println!("Another function!");
}

fn another_function_with_a_parameter(x: i32) {
    println!("Another function that outputs the argument {x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*Statements and Expressions
Statements  - Instructions that perform some action and do not return a value.
Expressions - Evaluate to a resultant value.
*/

fn statement() { //even fn definition is a statement
    let _y = 6;
}

fn expression() {
    let y = { // The let statement is assigned the value of the block expression
        let x = 3;
        x + 1 // no semi colon makes this an expression (a return value)
    }; // a block that evaluates to 4.

    println!("The value of y is: {y}");
}

fn function_with_return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
