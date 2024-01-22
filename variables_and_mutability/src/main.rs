const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    //Variables
    let x = 5;
    println!("The value of x is: {x}");

    //Mutating
    let mut y = 10;
    println!("The value of x is: {y}");

    y = 12;
    println!("The value of x is: {y}");

    //Constants
    println!("The global const value is {THREE_HOURS_IN_SECONDS}");

    const SIX_HOURS_IN_SECONDS: u32 = THREE_HOURS_IN_SECONDS * 2;
    println!("The scoped const value is {SIX_HOURS_IN_SECONDS}");

    //Shadowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}"); //12
    }

    println!("The value of z is: {z}"); //6

    let spaces = "    ";
    let spaces = spaces.len(); //type reallocated
    //You can't mut a shadowed type reallocation
    println!("The length of spaces is: {spaces}");
}
