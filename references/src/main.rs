fn main() {
    let mut s1 = String::from("hello");

    let mut len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    change(&mut s1);
    
    len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    // this block fails because you cannot have two mutable references to a value in the same scope
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");

    // this one would pass because it uses immutable references, until the third one mixes mutable
    // and immutable
    let v1 = &s; // no problem
    let v2 = &s; // no problem
    let v3 = &mut s; // BIG PROBLEM
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
