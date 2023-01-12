use std::io;
fn main() {
    let mut a:i32=0;
    let mut b:i32=0;
    let mut input = String::new();
    
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    a = input.trim().parse().expect("Not a valid number");
    
    for i in 1..=a{
        for j in 1..=(a-i)+1{
            print!("  ");
        }
        while b != (2*i-1){
            print!("* ");
            b += 1;
        }
        b=0;
        println!("");
    }
}