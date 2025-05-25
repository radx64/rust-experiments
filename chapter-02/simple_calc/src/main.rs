use std::io;

fn main() {
    let mut first_operand_input = String::new();
    let mut second_operand_input = String::new();
    let mut operator_input = String::new();

    println!("Input first operand");

    io::stdin()
        .read_line(&mut first_operand_input)
        .expect("Failed to parse input");

    let first_operand : f32 = first_operand_input.trim().parse().expect("Invalid number");

    println!("Input second operand");

    io::stdin()
        .read_line(&mut second_operand_input)
        .expect("Failed to parse input");

    let second_operand : f32 = second_operand_input.trim().parse().expect("Invalid number");

    println!("Input operation (+-*/)");

    io::stdin()
        .read_line(&mut operator_input)
        .expect("Failed to parse input");

    print!("Result: ");

    match  operator_input.trim(){
        "+" => println!("{first_operand} + {second_operand} = {}", first_operand + second_operand),
        "-" => println!("{first_operand} - {second_operand} = {}", first_operand - second_operand),
        "*" => println!("{first_operand} * {second_operand} = {}", first_operand * second_operand),
        "/" => {
            if second_operand != 0.0{
                println!("{first_operand} / {second_operand} = {}", first_operand / second_operand);
                } else {
                println!("Division by 0 is not allowed");
                }
    },
        _ => println!("invalid operator")
    };

}
