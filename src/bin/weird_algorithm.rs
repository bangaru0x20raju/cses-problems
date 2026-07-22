use std::io;
fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the number");
    let mut number: i64 = number.trim().parse().expect("Failed to parse the number");

    while number != 1 {
        print!("{number} ");
        if number & 1 != 0 {
            number = (number << 1) + number + 1;
        } else {
            number = number >> 1;
        }
    }
    println!("{number}");
}
