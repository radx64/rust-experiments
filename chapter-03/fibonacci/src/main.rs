use std::io;

fn main() {
    let n = read_n();

    let value = fib(n);

    println!("Nth: {n} of Fibonacci sequence has {value} value");

}

fn read_n() -> u32{
    let number = loop {
        println!("Which element of Fibonacci sequence You want to calculate:");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Could not read input");

        match number.trim().parse(){
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    number
}

fn fib(n : u32) -> u128{
    if n == 0 {
        0
    }
    else if n == 1 || n == 2 {
        1
    }
    else {
        fib(n-1) + fib(n-2)
    }
}
