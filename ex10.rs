use std::io;

fn print_point(name: &str, point: i32) {
    let mut i = 0;
    print!("{}:", name);
    loop {
        if i >= point {
            break;
        }

        print!("]");
        i += 1;

    }
    println!("");
}

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).ok();

    let elms:Vec<&str> = line.split_whitespace().collect();

    let a:i32 = elms[0].parse().unwrap();
    let b:i32 = elms[1].parse().unwrap();

    print_point("A", a);
    print_point("B", b);
}