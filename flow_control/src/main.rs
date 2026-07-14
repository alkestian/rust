fn main() {
    let x = 10;
    if x < 20 {
        println!("x is less than 20: x is {x}");
    } else {
        println!("x is 20 or greater: x is {x}");
    }

    let condition = true;
    let number = if condition { 3 } else { 6 };
    println!("Value of number is {number}");

    let mut i = 0;
    let result = loop {
        i += 1;

        if i >= 10 {
            // return the value after the break, and perform operations on
            break i * 2 + 5;
        }
    };

    println!("The loop broke on loop number {result}");

    // loop labelling
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // can remove the = to go from inclusive to exclusive
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
