fn main() {
    let a = rprompt::prompt_reply_stdout("Enter first number: ").unwrap();
    let b = rprompt::prompt_reply_stdout("Enter second number: ").unwrap();
    let cal = rprompt::prompt_reply_stdout("[1] Add [2] Subtract [3] Multiply [4] Divide: ").unwrap();

    let a_int: i32 = a.parse().unwrap();
    let b_int: i32 = b.parse().unwrap();
    let cal_int: i32 = cal.parse().unwrap();

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
