//Won't compile, this is intentional

/*
Ownership Rules:
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.
 */
fn main() {
    let mut s = String::from("hello"); //String allocated on heap from a string literal

    s.push_str(", world!"); //push_str() appends a literal to a String

    println!("{s}");
    let _x = s; //will copy the pointer, len, and capacity into x (they're on the stack, but leave the text on the heap)
                //rust frees up and invalidates s at this point, this is known as a "move" when is assigned a new variable

    println!("{s}"); //will error due to no longer being valid after x is instantiated (due to double free error)

    //So we'd say that s was moved into x

    //If we do want to deeply copy all the stack and heap data then we can do a clone
    let s1 = String::from("clone test");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    //Data types that have a known size at compile time, such as integers, will just clone by default (a copy) as the value is
    //on the stack. It is not done explicitly like with clone above, that is why its called "copy"

    let n1 = 5;
    let n2 = n1;

    println!("n1 = {n1}, n2 = {n2}"); //works!

    //When a function is called the same rules apply, a heap data type gets its ownership given to the variable in the function
    //and is invalid in the parent, a stack variable that can "copy()" will be valid in both parent and child function

    let f_s = String::from("Won't work after function as invalid");
    takes_ownership(f_s); // s invalidated
    println!("{f_s}"); //will panic due to being invalid after function call

    let f_n = 5;
    makes_copy(f_n);
    println!("{f_n}"); //will be okay due to copy

    let f_s_2 = String::from("A returned variable gives ownership back and so is valid in the parent");
    let f_s_2_returned = return_ownership(f_s_2);
    println!("{f_s_2_returned}"); //will be okay due to return

}

fn return_ownership(mine: String) -> String {
    mine
}

fn makes_copy(mine: i32) {
    println!("{mine}");
} // mine invalidated

fn takes_ownership(mine: String) {
    println!("{mine}");
} // mine invalidated