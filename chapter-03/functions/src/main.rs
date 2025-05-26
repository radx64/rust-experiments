fn main() {
    println!("Hello, world!");
    another_function();
    yet_another_function(5);
    print_labeled_measurement(5, 'h');

    // Statement (does not return value)
    let x = 6;
    // let x = (let y = 6); // this will not compile, statements does not return values
    println!("{x}");

    // Expression (returns value)
    let y = {
        let x = 3;
        x + 1               // notice that there is no ; here
    };
    println!("{y}");

    let x = five();
    println!{"{x}"};

    let x = plus_one(5);
    println!{"{x}"};
}

fn another_function(){
    println!("Another function.");
}

fn yet_another_function(x: i32)
{
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit: char)
{
    println!("The measurement is: {value}{unit}");
}

fn five() -> i32{
    5 // notice that there is no ; here
}

fn plus_one(x: i32) -> i32{
    x + 1 // notice that there is no ; here
}
