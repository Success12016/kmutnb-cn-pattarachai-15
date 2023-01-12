use std::io;
fn main() {
    let mut a:i32=0;
    let mut input = String::new();
    
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    a = input.trim().parse().expect("Not a valid number");

    for i in 1..=a{
        for j in 1..=a{
            if j==i || j==1{
                print!("X ");
            }else if j==a{
                println!("X");
            }else{
                print!("O ");
            }
        }
    }
}
