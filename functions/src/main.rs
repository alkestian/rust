fn main() {
    println!("Hello, world!");

    another_func();

    another_another_func(10);

    labeled_funcs(360, "degrees");

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    nested_func();

    let mut i = 0;
    while i < 10 {
        i = plus_one(i);
        println!("On loop number {i}");
    }
}

fn another_func() {
    println!("Other func!");
}

fn another_another_func(x: i32) {
    println!("x is {x}");
}

fn labeled_funcs(value: i32, unit_label: &str) {
    println!("The measurement is {value} {unit_label}")
}

fn four() -> i32 {
    4
}

fn nested_func() {
    println!("Linda: {}!", four());
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
