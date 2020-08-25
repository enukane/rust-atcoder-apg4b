use std::io;

fn main() {
    let mut acc:i32 = 1;
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    let elms: Vec<&str> = line.trim().split('1').collect();
    //println!("str => {:?}", elms);


    for elm in elms {
        match elm {
            "+" => acc = acc + 1,
            "-" => acc = acc - 1,
            "" => acc = acc,
            _ => panic!(),
        }
    }

    println!("{}", acc);

}