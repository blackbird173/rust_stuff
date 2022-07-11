// triangle char changer
use rand::Rng;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let alphabet = "qwertyuiopasdfghjklzxcvbnm".chars().collect::<Vec<char>>();
    let mut holder: Vec<usize> = vec![];
    let mut hold: Vec<char> = vec![];
    for i in 0..867 {
        if i % 101-100 == 0 {
            hold.push('\n');
        } else {
            hold.push('x');
        }
    }
    let mut start = 50;
    let mut end = 50;
    hold[50] = '.';
    for _ in 0..8 {
        start += 100;
        end += 102;
        hold[start] = '.';
        hold[end] = '.';
        for x in start..end+1 {
            hold[x] = '.';
        }
    }
    let mut count = 42;
    for i in 0..hold.len() {
        if i+1 < hold.len() && hold[i] == '.' && hold[i-count] == 'x' {
            for x in i-count..i {
                hold[x] = ' ';
            }
            count -= 1;
        }
    }
    hold = hold.into_iter().filter(|x| {
        if *x == 'x' {
            false
        } else {
            true
        }
    }).collect::<Vec<char>>();
    for i in 0..hold.len() {
        if hold[i] == '.' {
            holder.push(i);
        }
    }
    println!("{:?}", holder);
    loop {
        for i in holder.iter() {
            hold[*i] = alphabet[rand::thread_rng().gen_range(0..alphabet.len())];
        }
        println!("{}", hold.iter().collect::<String>());
        sleep(Duration::from_millis(100));
        Command::new("clear").status().unwrap();
    }
}
