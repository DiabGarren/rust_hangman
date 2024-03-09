use rand::Rng;
use std::io;

fn main() {
    let word_list: [&str; 52] = [
        "Apple", "Beach", "Brain", "Bread", "Brush", "Chair", "Chest", "Chord", "Click", "Clock",
        "Cloud", "Dance", "Diary", "Drink", "Earth", "Flute", "Fruit", "Ghost", "Grape", "Green",
        "Happy", "Heart", "House", "Juice", "Light", "Money", "Music", "Party", "Pizza", "Plant",
        "Radio", "River", "Salad", "Sheep", "Shoes", "Smile", "Snack", "Snake", "Spice", "Spoon",
        "Storm", "Table", "Toast", "Tiger", "Train", "Water", "Whale", "Wheel", "Woman", "World",
        "Write", "Youth",
    ];
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);

    let word = (word_list)[num];

    println!("Welcome to Hangman in Rust!");
    println!("Please enter a letter: ");
    let mut letter = String::new();
    io::stdin().read_line(&mut letter).unwrap();

    println!("{word}");
}
