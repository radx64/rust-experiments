fn main() {
    let v1 = vec![1, 2, 3];
    
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_iter2 = v1.iter();

    let total: i32 = v1_iter2.sum();
    assert_eq!(total, 6);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{v2:#?}");
}
