
fn non_mutable_borrow() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn mutable_borrow() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    //println!("Before calling closure: {list:?}"); // this would not compile as above there is a mutable borrow of list
    borrows_mutably();
    println!("After calling closure: {list:?}");   
}

fn main() {
    println!("Non mutable borrow example:");
    non_mutable_borrow();
    println!("Mutable borrow example:");
    mutable_borrow();
}
