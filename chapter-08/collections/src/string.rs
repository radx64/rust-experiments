pub fn do_string_stuff()
{
    let s = String::new();
    println!("{s}");

    let data = "initial contents";

    let s = data.to_string();
    println!("{s}");

    let s = "initial contents".to_string();
    println!("{s}");

    let greetings = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
        String::from("Cześć"),
    ];

    for greet in &greetings {
        println!("{greet}");
    }

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{s}");

    let baz = "baz";
    s.push_str(baz);
    println!("{s}");
    println!("{baz}"); // baz is still accessible so push str does not take ownership

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;

    println!("{s3}");
    println!("{s2}");
    // println!("{s1}"); this would not compile as s1 was moved to s3

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");  

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = String::from("Hola");
    let len = hello.len();
    println!("{hello} has {len} bytes");

    let hello = String::from("Здравствуйте");
    let len = hello.len();
    println!("{hello} has {len} bytes");

    let hello = String::from("Cześć");
    let len = hello.len();
    println!("{hello} has {len} bytes");

    // let character = hello[0];

    /*  slices in middle of a character panics
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");

    let hello = String::from("Cześć");
    let s = &hello[0..4];
    println!("{s}");
    */

    

}
