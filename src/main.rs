#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;
use std::io::prelude::*;

mod blockchain;

fn main() {
    print!("{}[2J", 27 as char);
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Please input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Please enter a valid integer!");
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop {
        print!("{}[2J", 27 as char);

        println!("Menu");
        println!("1) View blockchain");
        println!("2) New Transaction");
        println!("3) Mine block");
        println!("4) Change Difficulty");
        println!("5) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 =>
            {
                print!("{}[2J", 27 as char);

                println!("Exiting...");
                process::exit(0);
            },
            1 => {
                print!("{}[2J", 27 as char);

                for b in &chain.chain {
                    println!("{:#?}", &b);
                }

                write!(io::stdout(), "Press any key to continue...").unwrap();
                io::stdout().flush().unwrap();

                let _ = io::stdin().read(&mut [0u8]).unwrap(); // Read a single byte and discard
            },
            2 => {
                print!("{}[2J", 27 as char);

                let mut from = String::new();
                let mut to = String::new();
                let mut amount = String::new();

                print!("Enter address to send from: ");
                io::stdout().flush();
                io::stdin().read_line(&mut from);
                print!("Enter address to send to: ");
                io::stdout().flush();
                io::stdin().read_line(&mut to);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_tx(from.trim().to_string(),
                                        to.trim().to_string(),
                                        amount.trim().parse().unwrap());

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            },
            3 =>
            {
                print!("{}[2J", 27 as char);

                println!("Generating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully!"),
                    false => println!("Block generation failed!"),
                }
            },
            4 =>
            {
                print!("{}[2J", 27 as char);

                let mut new_diff = String::new();
                print!("Please enter difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty!"),
                    false => println!("Failed to update difficulty!"),
                }
            },
            5 =>{
                print!("{}[2J", 27 as char);

                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward!"),
                    false => println!("Failed to update reward!"),
                }
            }
            _ => println!("Invalid option please retry"),
        }

    }
}
