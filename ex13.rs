use std::io;

fn main() {
    let mut sum:i32 = 0;
    let mut line = String::new();
    let mut points: Vec<i32> = Vec::new();

    io::stdin().read_line(&mut line).ok();
    let n:i32 = line.trim().parse().unwrap();

    line = String::new();
    io::stdin().read_line(&mut line).ok();
    let apoints:Vec<&str> = line.split_whitespace().collect();

    for apoint_str in &apoints {
        let apoint:i32 = apoint_str.trim().parse().unwrap();
        sum = sum + apoint;
        points.push(apoint);
    }
    let avg = sum / n;

    for point in points {
        let diff:i32 = (avg - point).abs();
        println!("{}", diff);
    }

}