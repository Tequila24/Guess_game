extern crate rand;

use rand::*;
use std::string::*;
use std::io::*;
use std::fs::File;

struct GuessHolder {
    answer: String,
    placeholder: String,
}

impl GuessHolder {

    fn new(word: &String) -> Self {
        Self {
            answer: word.to_string(),
            placeholder: "_".repeat(word.len()),
        }
    }

    fn replace_if_contains(&mut self, character: &String) {

        for (i, c) in self.answer.chars().enumerate() {
            if c == character.chars().next().unwrap() {
                self.placeholder.remove(i);
                self.placeholder.insert(i, c);
            }
        }
    }

    fn is_done(&self) -> bool {
        if self.placeholder == self.answer {
            true
        } else {
            false
        }
    }
}

fn main() {
    let random_answer = get_random_answer_from_base();
    let mut guess = GuessHolder::new(&random_answer);

    println!("Word is {} chars long: {}", guess.placeholder.len(), guess.placeholder);

    while !guess.is_done() {

        let mut character = String::new();

        match std::io::stdin().read_line(&mut character) {
            Ok(_n) => {
                if character.len() > 1 {

                    character.truncate(1);

                    print!("\r");

                    println!("Your guess: {}", character);

                    guess.replace_if_contains(&character);

                    println!("{}", guess.placeholder);

                } else {
                    println!("Enter 1 symbol!");
                }
            }
            Err(error) => println!("Unknown symbol {}!", error),
        }
    }

    println!("You win!");
    
}

fn get_random_answer_from_base() -> String {

  let mut answers_base: Vec<String> = Vec::new();

    let f = File::open("base.txt");
    match f {
        Ok(handle) => {
            let mut lines = BufReader::new(&handle).lines();

            for (i, line) in lines.enumerate() {
                answers_base.push(line.unwrap());
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let random_number = thread_rng().gen_range(0, answers_base.len());

    let random_answer = answers_base[random_number].clone();

    random_answer
} 