fn main() {
    // demonstrating the annoyance of not having slices
    let mut s = String::from("hello world");

    let _first = &first_word_old(&s);
    s.clear();

    // a second word would now need a start and end index

    // instead, we can use slices:
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("{}", hello);
    // can omit the starting index
    let hello = &s[..5];
    println!("{}", hello);

    // can pass the len
    let len = s.len();
    let world = &s[6..len];
    println!("{}", world);
    // or omit the ending index
    let world = &s[6..];
    println!("{}", world);

    // now, the updated/correct func to find the first word
    let s = String::from("hello world");
    let first = first_word(&s);
    println!("{}", first);

    // we need to be aware the slice is using a borrowed value though
    // s.clear();
    // println!("{}", first);

    // by making the func take a str, we can pass String type or string literals/slices:
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";
    let word_from_string = first_word_slice(&my_string);
    println!("{}", word_from_string);
    let word_from_literal = first_word_slice(my_string_literal);
    println!("{}", word_from_literal);

    // we can also slice arrays of other types
    let a = [1, 2, 3, 4, 5];
    let subset = &a[1..4];
    assert_eq!(subset, &[2, 3, 4]);

    // this is just testing the output of a failed assertion
    assert_eq!(subset, &[1]);
}

fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// we edit the func to take a slice, there is no downside to this, and it provides flexibility
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
