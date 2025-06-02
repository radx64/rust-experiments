#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // ...
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America1"))
    }
    else {
        Some(format!("{state:?} is relatively new."))
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin : &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn print_cents(value : u8) -> String {
    if value < 2 {
        String::from("cent")
    }
    else {
        String::from("cents")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn main() {

    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(UsState::Alabama), Coin::Quarter(UsState::Alaska)];

    for coin in coins {
        let value = value_in_cents(&coin);
        println!("Coin {:?} is worth {} {}", coin, value, print_cents(value));
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}
