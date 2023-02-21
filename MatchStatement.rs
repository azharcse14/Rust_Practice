fn main(){
    let num = 70;
    match num{
        // single
        1 =>print!("one"),
        2 => println!("two"),
        // multiple 
        3 | 4 | 5 => println!(" 3 ,4 5"),
        // range 
        6..=10 => println!("range 6..10"),
        // default 
        _ => println!("non"),
    }
    // println!("{}",num);
}