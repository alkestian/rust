fn main() {
    // immutatable
    let _a = "hello";

    // Can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    // Both of these are allocated on the stack
    let x = 5;
    let _y = x;

    // Both are allocated to the same pointer on the heap, not a copy of the data for the second
    // variable
    let s1 = String::from("hello");
    let _s2 = s1;
    // to stop a double free error when these variables go out of scope,
    // Rust considers s1 as no longer valid after pointer is reassigned to another variable
    // hence:
    println!("{s1}");
    // throws a compile error, which is known as a move
}
