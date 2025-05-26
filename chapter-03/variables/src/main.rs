fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");


    let a: i32 = -1;
    let b: u32 = 1;
    
    let c: i64 = -2;
    let d: u64 = 2;

    let e: i128 = -3;
    let f: u128 = 3;

    let g: f32 = 3.0;
    let h: f64 = 6.0;

    let i: isize = 1;
    let j: usize = 1;

    println!("{},{},{},{},{},{},{},{},{},{}", a, b, c, d, e, f, g, h, i, j);

    let t = true;
    let r: bool = false;

    println!("{t}, {r}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");
}
