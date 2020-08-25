use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();

    let strs: Vec<&str> = s.split_whitespace().collect();
    let a:i32 = strs[0].parse().unwrap();
    let b:i32 = strs[1].parse().unwrap();

    println!("{}", a + b);


}
