use std::{io, thread::sleep, time::Duration, process::Command};
use rand::Rng;
fn main() {
   let mut line = String::new();
   for i in 0..26*7 {
      if i == (26*7)-1 {
         line.push(' ');
      } else if i == 26 {
         line.pop();
      }
      if i <= 26*3 || i > 26*4 {
         line.push(' ');
      } else if i > 26*3 {
         line.push('.');
      }
      if i == 26 || i == 26*2 || i == 26*3 || i == 26*4 || i == 26*5 || i == 26*6 || i == 26*7 {
         line.push('\n');
      }
   }
   let alphabet = "qwertyuiopasdfghjklzxcvbnm".chars().collect::<Vec<char>>();
   let mut lines = line.chars().collect::<Vec<char>>();
   loop {
      let mut inp = String::new();
      io::stdin().read_line(&mut inp).unwrap();
      let first = &inp.chars().collect::<Vec<char>>()[0];
      if alphabet.contains(first) {
         let mut pos = alphabet.iter().position(|&x| { x == *first }).unwrap() + (26*3)+3;
         if rand::thread_rng().gen_range(0..=1) % 2 == 0 {
            for _ in 0..3 {
               pos += 27;
               lines[pos] = '.';
               println!("{}", lines.iter().collect::<String>());
               sleep(Duration::from_millis(250));
               Command::new("clear").status().unwrap();
            }
         } else {
            for _ in 0..3 {
               pos -= 27;
               lines[pos] = '.';
               println!("{}", lines.iter().collect::<String>());
               sleep(Duration::from_millis(250));
               Command::new("clear").status().unwrap();
            }
         }
      }
      lines = line.chars().collect::<Vec<char>>();
   }
}
