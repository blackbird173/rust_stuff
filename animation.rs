// the entire point of this program is to mock ppl who thing this looks "hackery" and stuff so i wanted to make it cuz i was bored as hell
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
fn main() {
   let mut v = String::from("                                                            ").chars().collect::<Vec<char>>();
   let clrs = String::from("ợฬєгՇץยเ๏קคร๔Ŧﻮђןкɭչאςש๒ภ๓").chars().collect::<Vec<char>>();
   let mut pos = 34;
   let mut pos2 = 35;
   let mut pos3 = 37;
   let mut pos4 = 38;
   let e = pos2+1;
   v[pos] = '0';
   v[pos2] = '1';
   v[pos3] = '0';
   v[pos4] = '1';
   let mut b = true;
   let mut count = 0;
   print!("\x1b[32m");
   loop {
      if b {
         if count <= 10 {
            pos -= 1;
            pos2 -= 1;
            pos3 += 1;
            pos4 += 1;
            v[pos] = '0';
            v[pos2] = '1';
            v[pos3] = '0';
            v[pos4] = '1';
            if pos+1 != pos+2 {
               v[e] = clrs[rand::thread_rng().gen_range(0..clrs.len())];
               v[pos2+1] = clrs[rand::thread_rng().gen_range(0..clrs.len())];
               v[pos3-1] = clrs[rand::thread_rng().gen_range(0..clrs.len())];
            }
            count += 1;
         } else {
            b = false;
         }
      } else {
         if count >= 0 {
            pos += 1;
            pos2 += 1;
            pos3 -= 1;
            pos4 -= 1;
            v[pos] = '0';
            v[pos2] = '1';
            v[pos3] = '0';
            v[pos4] = '1';
            v[pos-1] = ' ';
            v[pos4+1] = ' ';
            count -= 1;
         } else {
            b = true;
         }
      }
      println!("{}", v.iter().collect::<String>());
      sleep(Duration::from_millis(100));
   }
}
