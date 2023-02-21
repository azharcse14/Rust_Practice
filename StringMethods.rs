fn main(){
    let name = "Azhar is my name".to_string();
    let l = name.len();
    println!("{}",l);
    let change =  name.replace("Azhar","Rubel");
    println!("{}",change);
    let mut name1 = "Azhar is my name".to_string();
     name1.push('s');
    println!("{}",name1);
    let mut  name_2 = "my cat name is".to_string();
    name_2.push_str(" Pankha");
    println!("{}",name_2);

}