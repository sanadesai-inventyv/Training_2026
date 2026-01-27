fn main() {
    println!("\n--- Using `loop` ---");
    let mut number = 0;
    loop {
        number += 1;

        if number == 3 {
            continue; 
        }

        println!("number: {}", number);

        if number >= 5 {
            println!("breaking loop at 5");
            break; 
        }
    }

    println!("\n--- Using `while` loop ---");
    let mut num = 0;
    while num < 6 {
        num += 1;

        if num % 2 == 0 {
            continue; 
        }

        println!("Odd number: {}", num);

        if num >= 5 {
            println!("breaking the while condition at 5");
            break;
        }
    }

    println!("\n--- Using `for` loop ---");
    for i in 1..=10 {
        if i == 4 {
            continue; 
        }

        println!("Value: {}", i);

        if i == 7 {
            println!("breaking the loop at 7");
            break; 
        }
    }

    println!("\nAll loops completed!");
}



