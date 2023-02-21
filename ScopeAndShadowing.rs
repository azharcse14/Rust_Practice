fn main(){

    // println!("{}",age);
    let age = 17;
    println!("{}",age);
    {
        // Shadowing allows you to re-declare a variable in the same scope, 
         // using the same name.
         let age = 20;
        let rank = 1;
        println!("{}",rank);
        println!("{}",age);
    }
    // println!("{}",rank);
    println!("{}",age);
}