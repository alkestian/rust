fn main() {
    // different ways to initialise a string
    let mut _s = String::new();

    let data = "inital contents";

    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let mut s = String::from("mut");
    s.push_str("bar");

    // s1 doesn't take ownership of s2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // but here s3 takes ownership of s1, s1 can no longer be used
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    // we borrow s2 because the + operator uses add(), which takes &str instead of String
    let s3 = s1 + &s2;
    println!("s3 is {s3}");
    println!("s1 is no longer borrowable");
    println!("s2 is {s2}");

    // push() appends char
    let mut s = String::from("lol");
    s.push('l');

    // concat
    // this way is unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = s1 + "-" + &s2 + "-" + &s3;

    // so instead we can do this
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // can't index into Strings in Rust
    let s1 = String::from("hi");
    let h = s1[0];
    // the reason is because Rust strings are UTF encoded, and so a "single char" could be many
    // bytes long
    //
    // we can instead use .char().nth() or similar
    // or create a string slice using ranges
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // because of the Unicode, there are two ways to iterate strings we must choose from:
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
