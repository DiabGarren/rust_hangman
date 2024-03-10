use rand::Rng;
use std::io;

fn main() {
    let words_list: [&str; 52] = [
        "apple", 
        "beach", 
        "brain", 
        "bread", 
        "brush", 
        "chair", 
        "chest", 
        "chord", 
        "click", 
        "clock",
        "cloud", 
        "dance", 
        "diary", 
        "drink", 
        "earth", 
        "flute", 
        "fruit", 
        "ghost", 
        "grape", 
        "green",
        "happy", 
        "heart", 
        "house", 
        "juice", 
        "light", 
        "money", 
        "music", 
        "party", 
        "pizza", 
        "plant",
        "radio", 
        "river", 
        "salad", 
        "sheep", 
        "shoes", 
        "smile", 
        "snack", 
        "snake", 
        "spice", 
        "spoon",
        "storm", 
        "table", 
        "toast", 
        "tiger", 
        "train", 
        "water", 
        "whale", 
        "wheel", 
        "woman", 
        "world",
        "write", 
        "youth",
    ];
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);

    let word: String = (words_list)[num].to_string();

    let mut guess_word: String = "_____".to_owned();

    let mut guesses = vec![];
    let mut incorrect = 0;

    println!("Welcome to Hangman in Rust!");
    loop {
        if guess_word.contains("_") {
            let mut display_word = vec![];

            println!("Please guess a letter: ");

            let mut letter = String::new();
            io::stdin().read_line(&mut letter).unwrap();
            let letter_clone = letter.clone();

            if guesses.contains(&letter) {
                println!("You have already guessed that letter!");
            } else {
                guesses.push(letter_clone);

                let mut correct = false;

                for i in 0..word.len() {
                    if guess_word.chars().nth(i).unwrap() == '_' {
                        if word.chars().nth(i).unwrap() == letter.chars().next().unwrap() {
                            display_word.push(word.chars().nth(i).unwrap());
                            correct = true;
                        } else {
                            display_word.push('_');
                        }
                    } else {
                        display_word.push(guess_word.chars().nth(i).unwrap());
                    }
                }

                guess_word = display_word.iter().collect();

                if !correct {
                    incorrect += 1;
                }
            }

            if !draw(guess_word.clone(), incorrect) {
                println!(" ______\n|      |\n|      O\n|     /|\\\n|     / \\\n|\n|\n|____\n{guess_word}\nSorry, you lost.");
                break;
            }
        } else {
            println!("Congratulations you won!");
            break;
        }
    }
}

fn draw(word: String, incorrect: usize) -> bool {
    let stages: [&str; 6] = [
        " ______\n|      |\n|\n|\n|\n|\n|\n|____",
        " ______\n|      |\n|      O\n|\n|\n|\n|\n|____",
        " ______\n|      |\n|      O\n|      |\n|\n|\n|\n|____",
        " ______\n|      |\n|      O\n|     /|\n|\n|\n|\n|____",
        " ______\n|      |\n|      O\n|     /|\\\n|\n|\n|\n|____",
        " ______\n|      |\n|      O\n|     /|\\\n|     /\n|\n|\n|____",
    ];
    if incorrect > stages.len() - 1 {
        false
    } else {
        let output: &str = (stages)[incorrect];
        println!("{output}\n{word}");
        true
    }
}
