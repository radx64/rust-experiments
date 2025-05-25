fn main() {
    let x = 5;
    let x = x + 2;
    
    {
        let x = x *2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
