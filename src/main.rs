use std::io;
use rand::Rng; // Dependency needs to be added to Cargo.toml


fn main() {
    println!("Hello! What is your name?");
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let name: String = input.trim().to_string();
    println!("Hello {}", name);
    println!("I'm thinking of a number between 1 and 100");
    println!("What is the number I am thinking of");

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..101);

    let mut guesses: i32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess: i32 = input.trim().parse().unwrap();

        guesses += 1;

        if  guess < num {
            println!("Your guess was too low")
        } else if  guess > num {
            println!("Your guess was too high")
        } else {
           println!("Congratulations {}! you guessed the number was {}", name, num);
			break
        }
    }
    println!("It took you {} guesses to do this", guesses);
}
