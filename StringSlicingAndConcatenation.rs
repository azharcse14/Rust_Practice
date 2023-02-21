fn main(){
    //Slicing
    let name = "Puspita";
    let show = &name[0..3];
    println!("{}",show);
    


    //Concatenate
    let name_2 = "Rafin";
    let name_3 = "HP".to_string();
    let add = format!("{} {}",name_2, name_3);
    println!("{}",add);

    //String Object
    // let name_obj = String::from("JAHDjg");
    let name_4 = "Sabit".to_string();
    let name_5 = " Boltu".to_string();
    let add_2 = name_4 + &name_5;
    println!("{}",add_2);


}