use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{tup:?}");

    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    let unit = ();
    println!("{unit:?}");

    let a = [1, 2, 3, 4, 5];
    println!("{a:?}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{months:?}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{a:?}");

    let b  = [3; 10];
    println!("{b:?}");

    let first = a[0];
    let second = a[1];
    println!("{first}, {second}");

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index : usize = index.trim().parse().expect("Index was not a number");

    let element = a[index];
    
    println!("The value of element at index {index} is: {element}");
}
