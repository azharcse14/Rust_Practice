fn main(){
    // as well
    let name:&str = "Google";
    println!("{}",name);
    //empty String
    let mut name_2 = String::new();
    //push value in empty String
    name_2.push_str( "Hmm");
    println!("{}",name_2);
    //Default String
    let name_3 = String::from("Default as");
    println!("{}",name_3);


}