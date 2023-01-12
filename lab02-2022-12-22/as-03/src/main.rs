use std::io;
fn main() {
    let mut a:i32=0;
    let mut input = String::new();
    
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    a = input.trim().parse().expect("Not a valid number");
    
    for i in (1..=a).rev(){
        for j in 0..=a-i{
            print!("  ");
        }
        for k in i..=2*i-1{
            print!("* ");
        }
        for k in 1..=i-1{
            print!("* ");
        }
        println!("");
    }
}