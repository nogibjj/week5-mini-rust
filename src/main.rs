use std::io;

fn int_to_roman(mut num: i32) -> String {
    let mut result = String::new();
    let values = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for (value, symbol) in values {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }
    result
}

fn main() {
    println!("Enter an integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim().parse::<i32>().unwrap();
    let result = int_to_roman(input);
    println!("The Roman numeral representation is: {}", result);
}
