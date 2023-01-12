use std::io;
fn main() {
    let mut a:i32=0;
    let mut input = String::new();
    
    println!("Enter a number: ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    a = input.trim().parse().expect("Not a valid number");

    print!("{} = ",a);
    for i in 0..=100{
        if a % 2 == 0{
            a = a/2;
            print!("2");
            if a != 1{
                print!("*");
            }else{
                break;
            }
        }else if a % 3 == 0{
            a = a/3;
            print!("3");
            if a != 1{
                print!("*");
            }else{
                break;
            }
        }else if a % 5 == 0{
            a = a/5;
            print!("5");
            if a != 1{
                print!("*");
            }else{
                break;
            }
        }else if a % 7 == 0{
            a = a/7;
            print!("7");
            if a != 1{
                print!("*");
            }else{
                break;
            }
        }else{
            print!("{}",a);
            break;
        }
    }
}