#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).unwrap();

    print!("Input a mining difficulty...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let diff = difficulty.trim().parse::<u32>().expect("Difficulty must be an integer!");

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice:");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address:");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut sender).unwrap();
                print!("Enter receiver address: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut receiver).unwrap();
                print!("Enter amount: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut amount).unwrap();

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed To Update Difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}
