use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let x: i32 = input.trim().parse().expect("");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("");
    let y: i32 = input.trim().parse().expect("");
    
    if x > 0 { 
        if y > 0 { println!("1") } else { println!("4"); }
    } else { 
        if y > 0 { println!("2") } else { println!("3"); }
    }
}
