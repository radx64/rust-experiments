mod guess;
use guess::Guess;

fn main() {
    let correct_guess = Guess::new(55);

    println!("{}", correct_guess.value());

    let panicing_guess = Guess::new(101);

    println!("{}", panicing_guess.value()); // this will not print as previous line will panic!
}
