fn main() {
    let word = String::from("Hello world");

    println!("{word}");

    println!("First word of '{word}' is {}", first_word(&word));

    println!("Second word of '{word}' is {}", second_word(&word));

    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");

    let a = [1, 2, 3, 4, 5, 6];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    println!("nice slice of {a:?} is {slice:?}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut num_of_hits = 0;
    let mut begining_index = 0;

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '
        {
            num_of_hits += 1;
            if num_of_hits == 1 {
                begining_index = i+1;
            }
            if num_of_hits == 2 {
                return &s[begining_index..i]
            }
        }
    }

    return &s[begining_index..]
}
