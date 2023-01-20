use {std::str::Split};

fn main() {
    let t: &str = "this cat this cat this rat";
    let s: Vec<&str> = t.split(" ").collect();
    let mut arr = vec![""];
    for j in 0..= s.len() {
        if arr.contains(j){
                s.remove(j);
            }
            arr[j] = j;
        }
        println!("Unique words:{}",s.len());
    }
        

