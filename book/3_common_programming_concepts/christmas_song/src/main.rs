// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

const DAY_NUMBERS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth",
    "eleventh", "twelvth",
];

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    for day_idx in 0..DAY_NUMBERS.len() {
        print_first_line(day_idx);
        print_gifts_for_day(day_idx);
        println!();
    }
}

fn print_first_line(idx: usize) {
    println!(
        "On the {} day of Christmas my true love gave to me:",
        DAY_NUMBERS[idx]
    );
}

fn print_gifts_for_day(idx: usize) {
    for gift_idx in (0..=idx).rev() {
        if idx != 0 && gift_idx == 0 {
            println!("and {}.", GIFTS[gift_idx])
        } else {
            println!("{},", GIFTS[gift_idx])
        }
    }
}
