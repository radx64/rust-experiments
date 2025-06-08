const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(&c.to_ascii_lowercase())
}

fn translate_to_pig_latin(sentence: &str) -> String {

    let mut translation = String::new();

    for word in sentence.split_whitespace() {
        let first_letter = word.chars().nth(0).unwrap_or(' ');
        if is_vowel(&first_letter) {
            println!("{word} starts with vowel");
            let new_word = format!("{}-hay", &word);
            translation.push_str(&new_word);
        }
        else {
            println!("{word} starts with consonant");
            let new_word = format!("{}-{}ay", &word[1..], &first_letter);
            translation.push_str(&new_word);
        }
        translation.push(' ');
    }
    
    translation
}

fn main() {
    let sentence = "Hello my apples are tasty";

    let pig_latin = translate_to_pig_latin(sentence);
    
    println!("Sentence:");
    println!("\t{sentence}");
    println!("in pig latin sound like:");
    println!("\t{pig_latin}");
}
