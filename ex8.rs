use std::io;

fn main() {
    let mut pattern_line = String::new();
    let mut price_line = String::new();
    let mut n_line = String::new();
    let mut text_line = String::new();

    io::stdin().read_line(&mut pattern_line).ok();

    let pattern:i32 = pattern_line.trim().parse().unwrap();

    if pattern == 2 {
        io::stdin().read_line(&mut text_line).ok();
    } else {
        return;
    }

    io::stdin().read_line(&mut price_line).ok();
    io::stdin().read_line(&mut n_line).ok();

    let price:i32 = price_line.trim().parse().unwrap();
    let n:i32 = n_line.trim().parse().unwrap();

    if !text_line.is_empty() {
        println!("{}", text_line.trim());
    }
    println!("{}", price * n);
}