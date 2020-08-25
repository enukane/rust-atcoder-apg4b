use std::io;

pub fn get_line(line: &mut String) {
    *line = String::new();
    io::stdin().read_line(line).ok();
}

pub fn get_number() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    let n:i32 = line.trim().parse().unwrap();

    return n;
}