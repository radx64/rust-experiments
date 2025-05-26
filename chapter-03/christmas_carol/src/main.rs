fn main() {
    
    let days = ["first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eight",
        "ninth", "tenth", "eleventh", "twelfth"];

    let presents = ["A partridge in a pear tree", "Two turtle doves", "Three french hens",
        "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming",
        "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping",
        "Twelve drummers drumming"];

    let mut day = 0;

    while day < days.len(){

        println!("On the {} day of Christmas", days[day]);
        println!("My true love gave to me");

        for present in (0..day+1).rev(){
            if present == 0 && day != 0 {
                println!("And {}", presents[present].to_lowercase())
            } else {
                println!("{}", presents[present])
            }
        }

        println!();
        day +=1;
    } 
    
}
