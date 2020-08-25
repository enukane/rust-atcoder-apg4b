use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();

    let strs: Vec<&str> = line.split_whitespace().collect();
    let a:i32 = strs[0].parse().unwrap();
    let op = strs[1];
    let b:i32 = strs[2].parse().unwrap();

    match op {
        "+" => println!("{}", a+b),
        "-" => println!("{}", a-b),
        "*" => println!("{}", a*b),
        "/" => if b == 0 {
            println!("error");
        } else {
            println!("{}", a/b);
        },
        _ => println!("error"),
    }
}
