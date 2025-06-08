pub fn do_vector_stuff() {
    let v: Vec<i32> = Vec::new();
    println!("v contains {v:?}");

    let v2 = vec![1,2,3,4];
    println!("v2 contains {v2:?}");

    let mut v3 = Vec::new();  // see that type annotation is not needed
    v3.push(5);                         // as push is using i32 type, so it can be inferred
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v3 contains {v3:?}");

    let yet_another_v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &yet_another_v[2];

    println!("The third element of yet_another_v is {third}");

    let third: Option<&i32> = yet_another_v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    
    /*
    let broken_access_v = vec![1, 2, 3, 4, 5];

    let does_not_exists = &v[100];      // this panics
    let does_not_exists = v.get(100);   // this returns None type
    */

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //v.push(6);      //  this invalidates reference above as vector can reallocate

    println!("The first element is: {first}");

    for i in &mut v {
        *i +=50;
    }

    println!("{v:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int((3)),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.21),
    ];

    println!("row elements are {row:?}");
    
}
