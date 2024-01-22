use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5]; //allocates on the stack instead of heap
    let _b: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let _c = [3; 4]; // [3,3,3,3]

    let _first = a[0];
    let _second = a[1];

    println!("Please enter and array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: u8 = a[index];

    println!("The value of the element at index {index} is: {element}");
}
