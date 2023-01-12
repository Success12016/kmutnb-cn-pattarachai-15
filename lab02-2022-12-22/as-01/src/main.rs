use std::io;
fn main() {
    let mut a:i32=0;
    let mut input = String::new();
    
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    a = input.trim().parse().expect("Not a valid number");
    
    for i in 1..=a{
        for j in a-i..=a-1{
            if j != a-1{
                print!("* ");
            }else{
                println!("*");
            }
        }
    }
}