#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 7},
        Rectangle {width: 7, height: 12},
    ];

    println!("{list:#?}");
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    let mut another_list = list.clone();

    let mut num_sort_operations = 0;

    another_list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{list:#?}, sorted in {num_sort_operations} operations");

}
