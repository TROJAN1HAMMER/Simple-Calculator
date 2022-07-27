use std::io::Write;

fn main() {
    let mut a = String::new();
    println!("Enter first number: ");
    std::io::stdout().flush().unwrap(); // Console output must be flushed to remove errors.
    std::io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    println!("Enter second number: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut b).unwrap();

    println!("[1] Add [2] Subtract [3] Multiply [4] Divide: ");
    std::io::stdout().flush().unwrap();
    let mut cal = String::new();
    std::io::stdin().read_line(&mut cal).unwrap();

    let a_int: i32 = a.trim().parse().unwrap();
    let b_int: i32 = b.trim().parse().unwrap();
    let cal_int: i32 = cal.trim().parse().unwrap();

    let mut ans = 0;

    if cal_int == 1 {
        ans += a_int + b_int;
    } else if cal_int == 2 {
        ans = a_int - b_int;
    } else if cal_int == 3 {
        ans = a_int * b_int;
    } else if cal_int == 4 {
        ans = a_int / b_int;
    }

    println!("The answer is {}", ans)
}