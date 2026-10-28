fn main() {
    // can tell it the type before adding values
    let _v: Vec<i32> = Vec::new();

    // or let it infer the types
    let mut v = vec![1, 2, 3];

    //to add values, we .push()
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element"),
    }

    // causes index out of bounds panic
    // let _does_not_exist = &v[100];

    // returns None without panicking
    let does_not_exist = &v.get(100);
    // proof of None
    match does_not_exist {
        Some(_does_not_exist) => println!("How the fuck did we get here?"),
        None => println!("This element doesn't exist, cause it's 95 indexes out of bounds."),
    }

    v.push(6);

    // if we do an immutable borrow
    // let first = &v[0];
    // we can't do a muutable borrow
    //v.push(7);
    // while the immutable is still in scope and borrowed
    //println!("The first element is: {first}");

    // Iterating
    // for loop
    for i in &v {
        println!("{i}");
    }

    // mutable changes via for loop
    for i in &mut v {
        *i *= 50;
        println!("{i}");
    }

    // can't affect the vector while borrowing in the loop
    // for i in &mut v {
    //     *i += 1;
    //     v.push(*i);
    // }

    // can use enums to have different data types in one vector
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }
    //
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // finally, when a vector goes out of scope, its elements are dropped too
    // but otherwise we can use basic commands:

    v.pop(); // remove last element
    v.remove(0); // remove at index
    v.truncate(3); // truncate up to nth element

    for i in &v {
        println!("{i}");
    }
}
