fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let greetingng = guess.contains("Hello");
    if greetingng {
        println!("Your guess contains 'Hello'");
    } else {
        println!("Your guess does not contain 'Hello'");
    }
    
}


/* fn main() {
    println!("Hello, world!");
}
 */

