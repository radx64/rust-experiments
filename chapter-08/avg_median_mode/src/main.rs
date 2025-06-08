
fn get_median(vec: &Vec<i32>) -> i32 {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();

    println!("sorted: {sorted_vec:?}");

    let middle_index = sorted_vec.len() /2;
    let even = sorted_vec.len() % 2 == 0;

    let median = match even {
        true => (sorted_vec[middle_index-1] + sorted_vec[middle_index]) / 2,
        false => sorted_vec[middle_index]
    };

    median
}

use std::collections::HashMap;

fn get_mode(vec: &Vec<i32>) -> Vec<i32> {

    let mut map: HashMap<i32, i32> = HashMap::new();

    for element in vec {
        let counter = map.entry(element.clone()).or_insert(0);
        *counter +=1;
    }

    let mut result: Vec<i32> = Vec::new();

    let mut mod_count = 0;

    for (key, value) in &map {
        if *value > mod_count {
            result.clear();
            mod_count = *value;
            result.push(*key);
        } else if *value == mod_count  {
            result.push(*key);
        }
    }

    println!("{map:#?}");

    result
}

fn main() {

    let list = vec![1, 4, 7, 3, 3, 3, 2, 6, 8, 19, 100, 2, 4, 2];
    println!("{list:?}");

    println!("Median is {} (integer)", get_median(&list));
    println!("Mode (dominant) is {:?}", get_mode(&list));

    println!("");


}
