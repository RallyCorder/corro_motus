use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;

const WORD_LIST: &str = include_str!("words.txt");
const WORD_LENGHT: usize = 5; //change depending on desired lenght
const ATTEMPTS_POSSIBLE: usize = 10;

fn words() -> Vec<String> {
    WORD_LIST
        .split('\n')
        .skip(1)
        .map(|s| s.trim().to_string())
        .collect()
}

struct Game {
    dictionnary: Vec<String>,
    word: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl Game {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let dictionnary = words();
        let word = rng.random_slice_entry(&dictionnary).unwrap().clone();
        Self {
            dictionnary,
            word,
            guessed_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }

    fn display_guesses(&mut self) {
        self.guesses.iter().enumerate().for_each(|(guess_nbr, guess)| {
            print!("{}: ", guess_nbr + 1);
            guess.chars().enumerate().for_each(|(pos, c)| {
                let display = if let Some(word_char) = self.word.chars().nth(pos) {
                    if word_char == c {
                        format!("{}", c).bright_yellow()
                    } else if self.word.chars().any(|wc| wc == c) {
                        format!("{}", c).bright_purple()
                    } else {
                        self.guessed_letters.insert(c);
                        format!("{}", c).red()
                    }
                } else {
                    format!("{}", c).red()
                };
                print!("{}", display);
            });
            println!();
        });
    }

    fn display_invalids(&self) {
        if !self.guessed_letters.is_empty() {
            print!("{}", format!("Characters not included: ").bright_green());
            self.guessed_letters.iter().for_each(|letter| print!("{}", format!("{letter} ").bright_red()));
            println!();
        }
    }

    fn user_input(&mut self) -> String {
        println!(
            "{}",
            format!(
                "『 Enter your guess [{}] and press Enter 』",
                WORD_LENGHT
            ).bright_green()
        );
        self.display_invalids();
        let mut guess = String::new();
        let mut valid_guess = false;
        while !valid_guess {
            guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            guess = guess.trim().to_string();
            if guess.len() != WORD_LENGHT {
                println!("{}", format!("Your input must be of {} characters", WORD_LENGHT).red());
            } else if !self.dictionnary.iter().any(|word| word == &guess) {
                println!("{}", format!("{} isn't in the word list :P", guess).red());
            } else {
                self.guesses.push(guess.clone());
                valid_guess = true;
            }
        }
        guess
    }

    fn end_scenario(&self, guess: &str) -> bool {
        let n_tries = self.guesses.len();
        if guess == self.word {
            println!("Congratulations! You tried {} and succeeded.", n_tries);
            true
        } else if n_tries >= ATTEMPTS_POSSIBLE {
            println!("{}", format!("Это... The word was {}. Revise your Oxford and come back soon!", self.word).bright_green());
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut game = Game::new();
    println!("{}", format!("Corro-Motus").on_bright_green().black());
    loop {
        game.display_guesses();
        let guess = game.user_input();
        if game.end_scenario(&guess) {
            break;
        }
    }
}
