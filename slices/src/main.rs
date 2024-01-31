fn main() {
    let s = String::from("Hello, world!");

    let _slice = &s[0..2];
    let _slice = &s[..2]; // these mean the same thing

    let s_len = s.len();
    let _slice = &s[3..s_len];
    let _slice = &s[3..]; //these also mean the same

    let _slice = &s[0..s_len];
    let _slice = &s[..]; //these also mean the same

    let word = first_word(&s);
    println!("{word}");

    //str literals are just slices referencing to a point in binary
    //A String type with a '&' reference is just a reference to a str in binary which is a slice, so its interchangeable
    let n = "string literal";
    let word = first_word_with_str(&s);
    println!("{word}");
    let word = first_word_with_str(n);
    println!("{word}");
    let word = first_word_with_str(&s[..5]);
    println!("{word}");

    //slices apply to arrays as well
    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    println!("Slice of array: {slice:?}");
}

//type of the parameter should be &str because it will work for both str and String types due to
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_with_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}