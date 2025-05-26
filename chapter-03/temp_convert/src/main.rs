use std::io;

fn main() {
    'main: loop {
        let option = get_option();
        println!("You've entered {option}");
        
        if option == 3 { break 'main; }

        let temperature = get_temperature();

        if option == 1 {
            println!{"Temperature {temperature}*F is {}*C", (temperature - 32.0) / 1.8}
        }else{
            println!{"Temperature {temperature}*C is {}*F", temperature * 1.8 + 32.0}
        }
        println!("------");
    }

}

fn get_option() -> u32 {
    let mut option = String::new();

    let option: u32 =  loop {
        println!("Select option:");
        println!("1 - Fahrenheit to Celsius");
        println!("2 - Celsius to Fahrenheit");
        println!("3 - Exit");

        option.clear();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim().parse(){
            Ok(num) => break match num {
                1 | 2 | 3 => num,
                _ => continue,
            },
            Err(_) => continue
        };
    };
    
    option
}

fn get_temperature() -> f32{
    let mut temperature = String::new();

    let temperature: f32 = loop {
        println!("Enter temperature to convert: ");
        temperature.clear();
        io::stdin().read_line(&mut temperature).expect("Failed to read temp");

        match temperature.trim().parse(){
            Ok(num) => break num,
            Err(_) => continue
        }
    };

    temperature
}
