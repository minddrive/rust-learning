fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "eleventh", "twelfth"
    ];
    let gifts = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for day in 1..=days.len() {
        println!("On the {} day of Christmas my true love gave to me", days[day - 1]);

        if day == 1 {
            println!("A partridge in a pear tree");
            println!();
        } else {
            for countdown in (0..day).rev() {
                println!("{}", gifts[countdown]);
            }
            println!();
        }
    }
}
