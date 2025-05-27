fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{s}");

    // Move s1 to s2. s1 is no longer valid
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!");  // this would not compile

    let mut s = String::from("hello");
    s = String::from("ahoy");   // s drop is called from previous String instance

    // Clone copies object
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    println!("{s}, world!");
}
