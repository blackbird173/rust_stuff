use std::{io, io::Write, fs::OpenOptions};
#[derive(Debug)]
enum Prizes {
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
   println!("spinning...");
   let mut word = String::from(words[&0 as *const i32 as usize % 7 as usize]);
   let mut blank = vec!['-'; word.len()];
   println!("the word has {} chars and every wrong guess makes ur money go down by 10 dollars (the word is a food and it starts with a {})", word.len(), word.chars().collect::<Vec<char>>()[0]);
   let backup = word.clone();
   let mut money = 10000000;
   loop {
      println!("money: {} and word: {}", money, blank.iter().collect::<String>());
      if backup == blank.iter().collect::<String>() {
         if money <= 100 {
            println!("ur prize is a {:?} and {} dollars", Prizes::Plushie, money);
         } else if money <= 9999949 {
            println!("ur prize is a {:?} and {} dollars", Prizes::RubberDucky, money);
         } else if money <= 9999959 {
            println!("ur prize is a {:?} and {} dollars", Prizes::Fan, money);
         } else if money <= 9999969 {
            println!("ur prize is a {:?} and {} dollars", Prizes::BathTub, money);
         } else if money <= 9999979 {
            println!("ur prize is a {:?} and {} dollars", Prizes::InDoorTentBed, money);
         } else if money <= 9999989 {
            println!("ur prize is a {:?} and {} dollars", Prizes::Boat, money);
         } else if money >= 9999999 {
            println!("ur prize is a {:?} and {} dollars", Prizes::Car, money);
         }
         let end = "u wont ".to_owned() + money.to_string().as_str() + " and a prize\n";
         let mut file = OpenOptions::new().create(true).append(true).open("wheel_of_food.txt").unwrap();
         file.write_all(end.as_bytes()).unwrap();
         break;
      }
      let mut guess = String::new();
      io::stdin().read_line(&mut guess).unwrap();
      let _ = guess.trim().chars().map(|x| {
         if word.find(x) != None {
            if word.contains(x) {
               blank[word.find(x).unwrap()] = x;
               let mut vec_word = word.chars().collect::<Vec<char>>();
               vec_word[word.find(x).unwrap()] = '-';
               word = vec_word.iter().collect::<String>();
            }
         }
         if !backup.contains(x) {
            money -= 10;
         }
         x
      }).collect::<String>();
   }
}
