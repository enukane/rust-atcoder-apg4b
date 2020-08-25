fn main() {
    let a = true;
    let b = false;
    let c = true;
    
    if a == true {
        print!("At");
    } else {
        print!("Yo");
    }

    if !a && b {
        print!("Bo");
    } else if !b || c {
        print!("Co");
    }

    if a && b && c {
        print!("foo!");
    } else if true && false {
        print!("yeah!");
    } else if !a || c {
        print!("der");
    }
    println!("");
}
