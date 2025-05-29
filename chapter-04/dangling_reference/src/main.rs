fn main() {
    //let reference_to_nothing = dangle();
    let copy = copy();

    println!("{copy}");

}

fn copy() -> String{
    let s = String::from("hello");
    
    s
}

// fn dangle() -> &String{
//     let s = String::from("hello");
    
//     &s
// }
