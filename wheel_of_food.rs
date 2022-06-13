// This program allows the user to play a game and win prizes by guessing the correct word.
use std::{io, io::Write, fs::OpenOptions};
#[derive(Debug)]
enum Prizes {
   // This enums stores all the possible prizes.
   Plushie,
   RubberDucky,
   Fan,
   BathTub,
   InDoorTentBed,
   Boat,
   Car,
}
fn main() {
   let words = vec!["cheese", "sauce", "pasta", "grapes", "orange", "salad", "taco"];
   // These are the possible words that can be chosen.
   println!("spinning...");
   let mut word = String::from(words[&0 as *const i32 as usize % 7 as usize]);
   let mut blank = vec!['-'; word.len()];
   println!("the word has {} chars and every wrong guess makes ur money go down by 10 dollars (the word is a food and it starts with a {})", word.len(), &word[0..1]);
   let backup = word.clone(); // This variable stores a copy of the word for the game.
   let mut money = 10000000; // This is the starting money.
   loop {
      println!("money: {} and word: {}", money, blank.iter().collect::<String>());
      if backup == blank.iter().collect::<String>() {
         match money { //This matches all the possible rewards to the money.
            0..=9999948 => {
               println!("ur prize is a {:?} and {} dollars", Prizes::Plushie, money);
            },
            9999949..=9999958  => {
               println!("ur prize is a {:?} and {} dollars", Prizes::RubberDucky, money);
            },
            9999959..=9999968  => {
               println!("ur prize is a {:?} and {} dollars", Prizes::Fan, money);
            },
            9999969..=9999978  => {
               println!("ur prize is a {:?} and {} dollars", Prizes::BathTub, money);
            },
            9999979..=9999988  => {
               println!("ur prize is a {:?} and {} dollars", Prizes::InDoorTentBed, money);
            },
            9999989..=9999998  => {
               println!("ur prize is a {:?} and {} dollars", Prizes::Boat, money);
            },
            9999999..=10000000 => {
               println!("ur prize is a {:?} and {} dollars", Prizes::Car, money);
            },
            _ => {},
         }
         let end = "u won ".to_owned() + money.to_string().as_str() + " and a prize\n";
         let mut file = OpenOptions::new().create(true).append(true).open("wheel_of_food.txt").unwrap(); // This creates a new file for appending your winning stats.
         file.write_all(end.as_bytes()).unwrap();
         break;
      }
      let mut guess = String::new(); // This creates a guess for what the letters could be.
      io::stdin().read_line(&mut guess).unwrap();
      let _ = guess.trim().chars().map(|x| { // This iters through guess and matches letters to the word and reveals them.
         if word.find(x) != None {
            if word.contains(x) {
               blank[word.find(x).unwrap()] = x;
               let mut vec_word = word.chars().collect::<Vec<char>>();
               vec_word[word.find(x).unwrap()] = '-';
               word = vec_word.iter().collect::<String>();
            }
         }
         if !backup.contains(x) { // If it is a wrong letter it subtracts money.
            money -= 10;
         }
         x
      }).collect::<String>();
   }
}
