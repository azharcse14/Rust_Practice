fn main(){
    // let one = 1;
    // let two:i8 = 10;
    // let three = 20_i8;
    // println!("{}",three);
    // let four = 12.2_f32;
    // println!("{}",four);
    let five = 1i32;
    println!("{} - byte",std::mem::size_of_val(&five));
    // let six = 2i16;
    // println!("{}",std::mem::size_of_val(&six));
    let seven = five as f32;
    println!("{} - byte",std::mem::size_of_val(&seven));
    println!("{}",five);
    println!("{}",seven);
}