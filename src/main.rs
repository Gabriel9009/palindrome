use std::io;
fn main(){
    let mut char_check : usize = 0;
    let mut char_check1 : usize = 1;
    println!("Enter a word");
    let mut userinput :String = String::new();
    io::stdin().read_line(&mut userinput).expect("n");
    let userinput: String = userinput.trim().parse().expect("j");

    let mut store_char: Vec<char> = Vec::new();
    let store_char: Vec<char> = userinput.chars().collect();

    let mut count_middle = 0;
    let mut limit_count = store_char.len() / 2;

    for a in 0..store_char.len(){
        if store_char[char_check] == store_char[store_char.len() - char_check1] {
            char_check+= 1;
            char_check1 += 1;
            count_middle+= 1;
            if count_middle == limit_count{
                println!("{} is a palindrome", userinput);
            }
        }else{
            println!("{} is not a palindrome", userinput);
            break;
        }
    }
   
}
