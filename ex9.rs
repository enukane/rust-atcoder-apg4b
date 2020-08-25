use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).ok();

    let strs:Vec<&str> = line.split_whitespace().collect();

    let x:i32 = strs[0].parse().unwrap();
    let a:i32 = strs[1].parse().unwrap();
    let b:i32 = strs[2].parse().unwrap();

    let ans_1 = x + 1;
    println!("{}", ans_1);

    let ans_2 = ans_1 * (a+b);
    println!("{}", ans_2);

    let ans_3 = ans_2 * ans_2;
    println!("{}", ans_3);

    let ans_4 = ans_3 - 1;
    println!("{}", ans_4);
}