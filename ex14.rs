use std::io;

fn main() {
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;
    let mut line = String::new();

    io::stdin().read_line(&mut line).ok();
    let elms: Vec<&str> = line.split_whitespace().collect();

    for elm in &elms {
        let val:i32 = elm.parse().unwrap();
        if val > max {
            max = val;
        }
        if val < min {
            min = val;
        }
    }

    println!("{}", max - min);
}