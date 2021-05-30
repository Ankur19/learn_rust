fn main() {
    println!("Hello, world!");

    another_function();

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The number after condition: {}", number);
}

fn another_function() {
    println!("Another function.");
}
