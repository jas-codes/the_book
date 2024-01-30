fn main() {
    //method using tuple
    let s1 = String::from("hello tuple string");
    let (s2, len1) = calculate_length_using_tuple(s1);
    println!("The length of string: {s2}, is {len1}");

    //method using reference
    let s1 = String::from("hello reference string");
    let len1 = calculate_length_using_reference(&s1);
    println!("The length of string: {s1}, is {len1}");

    //method using a mutable reference
    let mut s1 = String::from("Have you been changed?");
    println!("{s1}");
    changeable_reference(&mut s1);
    println!("{s1}");

    //can't have multiple mutable references to the same variable at once
    let _r1 = &s1;
    // let r2 = &mut s1;

    {
        let _r2 = &mut s1; // This works though because it's in a different scope and invalidated before r1 could be used
    }

    // The above stops data races, where one reference tries to update the value and the other tries to read from it
    // Rust does the same with immutable + a mutable reference
    // let r3 = &mut s1;

    //You can have as many immutable references as you like
    let s3 = String::from("many immutable references");
    let r4 = &s3;
    let r5 = &s3;
    println!("{r4}, {r5}");

    //Rust doesn't allow you to create dangling references. Which is a reference to a variable that has already gone out of scope
    //the compiler will detect it for you
}

fn calculate_length_using_tuple(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_using_reference(s: &String) -> usize { //s refers to s1 which references to the literal on the heap
    s.len()
}// Here, s goes out of scope. But, because it does not have ownership of what it refers to, the String is not dropped

//The act of creating a reference is called "Borrowing"
//references are immutable by default, you have to add mut to make it changeable
fn changeable_reference(s: &mut String) {
    s.push_str(" I was changed!");
}