use rand::Rng;
use std::io;

fn main() {
    println!("\n🦀 Matching Pennies in Rust! 🦀\n");
    println!("This program allows you to play matching pennies against a computer opponent");
    println!("The game will last for 5 rounds");
    println!("At the start of a round, each player picks \"heads\" or \"tails\".");
    println!("If the choices match, you win (+1 to you, -1 to computer)");
    println!("If the choices do not match, you lose (-1 to you, +1 to computer)\n");

    let mut rounds: i8 = 1;
    let mut player_score: i8 = 0;
    let mut cpu_score: i8 = 0;

    // this will be our input string
    let mut h_or_t = String::new();

    while rounds <= 5 {
        println!("Beginning round {}", rounds);

        'inputloop: loop {
            println!("Enter \"heads\" or \"tails\": ");

            // Take user input
            io::stdin()
                .read_line(&mut h_or_t)
                .expect("Failed to read line");

            match h_or_t.trim() {
                "heads" => break 'inputloop,
                "tails" => break 'inputloop,
                _ => {
                    println!("Incorrect input");
                    h_or_t = String::new();
                    continue;
                }
            }
        }

        // generate a random integer, 1 or 2
        let cpu_result: i32 = rand::thread_rng().gen_range(1..2);
        let mut cpu_choice = String::new();

        // turn random value into string result
        match cpu_result {
            1 => cpu_choice = String::from("heads"),
            2 => cpu_choice = String::from("tails"),
            _ => println!("Input error. Must be \"heads\" or \"tails\""),
        }

        println!("You chose {} and the CPU chose {}\n", h_or_t, cpu_choice);

        // match the user and cpu input
        if h_or_t.trim() == cpu_choice.trim() {
            player_score += 1;
            cpu_score -= 1;
            println!("You win!");
            h_or_t = String::new();
        } else {
            player_score -= 1;
            cpu_score += 1;
            println!("You lose!");
            h_or_t = String::new();
        }

        println!("Round {} over\n", rounds);
        rounds += 1;
        
        // end game summary
        if rounds > 5 {
            println!("All rounds are complete");
            println!("Your score: {}", player_score);
            println!("Opponent score: {}", cpu_score);
        }
    }
}
