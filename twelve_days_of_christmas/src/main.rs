const DAYS: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
    "eighth", "ninth", "tenth", "eleventh", "twelfth"];

const GIFTS: [&str; 12] = ["A partridge in a pear tree", "turtle doves", "french hens", "calling birds",
    "gold rings", "geese a-laying", "swans a-swimming", "maids a-milking", "ladies dancing",
    "lords a-leaping", "pipers piping", "drummers drumming"];

fn main() {
    for num in 0..GIFTS.len() {
        println!("On the {} day of Christmas, my true love gave to me", DAYS[num]);

        gift_line(num);

        if num == 0 { continue };

        for j in (0..num).rev() {
            gift_line(j);
        }

    }
}

fn gift_line(num: usize) {
    if num != 0 {
        println!("{} {}", num + 1, GIFTS[num]);
    }
    else {
        println!("{}", GIFTS[num]);
    }
}
