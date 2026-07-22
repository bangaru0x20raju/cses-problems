use std::io;
fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the number");
    let n: i32 = number.trim().parse().expect("Failed to parse the number");
    let mut temp_sum: i64 = 0;
    number.clear();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the number");
    for str_digit in number.trim().split(" ") {
        let digit: i64 = str_digit.parse().expect("Failed to parse the number");
        temp_sum += digit;
    }

    let total_sum = (n as i64 * (n as i64 + 1)) / 2;
    println!("{}", total_sum - temp_sum);
}
