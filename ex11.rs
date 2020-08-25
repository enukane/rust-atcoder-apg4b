use std::io;

fn main() {
    let mut acc: i32;

    let mut line = String::new();

    // get N
    io::stdin().read_line(&mut line).ok();
    let n:i32 = line.trim().parse().unwrap();

    // get A
    line = String::new();
    io::stdin().read_line(&mut line).ok();
    acc = line.trim().parse().unwrap();

    for i in 0..n {
        line = String::new();
        io::stdin().read_line(&mut line).ok();
        let elms: Vec<&str> = line.split_whitespace().collect();

        let op:&str = elms[0];
        let val:i32 = elms[1].parse().unwrap();

        match op {
            "+" => acc = acc + val,
            "-" => acc = acc - val,
            "*" => acc = acc * val,
            "/" =>
                if val == 0 {
                    println!("error");
                    break;
                } else {
                    acc = acc / val;
                },
            _ => {
                println!("error");
                break;
            },

        }


        println!("{}:{}", i+1, acc);
    }

}