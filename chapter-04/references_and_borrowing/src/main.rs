fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of 's1' is {len}.");

    change(&mut s1);

    let len = calculate_length(&s1);

    println!("The length of 's1' is {len}.");

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("{r2}");

    // Below will not compile, you can't have mutable and immutable references at the same time
    // If they are used in the same scope
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // But this one complies as r1 and r2 are used before r3 is created
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.
    // so its scopes ends (not on block end but on usage end)

    let r3 = &mut s; // no problem
    println!("{r3}");

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // this would not compile as borrow reference is not mutable
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
