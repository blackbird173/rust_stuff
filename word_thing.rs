// This program allows the user to enter a word and a little guy carries the word char by char in reverse to form the word.
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
use std::io::stdin;
fn main() {
    Command::new("clear").status().unwrap(); // Clears the terminal.
    println!("enter a word");
    let mut inp = String::new(); // To enter the word.
    stdin().read_line(&mut inp).unwrap(); // Reads user input.
    let wword = " ".to_owned() + inp.trim(); // Creates a new var that is used for getting the chars.
    let mut count = 0;
    let mut times = wword.len()-1; // How many times the little guy is gonna go back and forth.
    let w = wword.chars().collect::<Vec<char>>(); // Turns the word into a vector so chars can be changed with ease.
    let mut big = String::from(wword.chars().map(|mut _x| { _x = ' '; _x }).collect::<String>() + "   " + "\n"); // Creates a space where the little guy can move.
    for _ in 0..wword.len()+3 {
        big.push(' '); // Adds to the big variable.
    };
    let mut pos = big.len()-2; // Gets the pos of the little guy.
    let mut big = big.chars().collect::<Vec<char>>(); // Converts big into a vector so each element can be modified.
    big[pos+1] = '|';
    big[pos] = '0';
    big[pos-1] = '|';
    let mut c = 0;
    for i in 0..big.len() {
        if big[i] == '\n' {
            c = i;
        }
    }
    while times != 0 {
        if big[pos-2] == ' ' {
            let _ = big.iter().map(|x| { print!("{}", x); *x}).collect::<Vec<char>>(); // Prints each char of big.
            sleep(Duration::from_millis(100));
            Command::new("clear").status().unwrap();
            pos -= 1;
            big[pos+1] = '|';
            big[pos] = '0';
            big[pos-1] = '|';
            big[pos+2] = ' ';
        } else {
            while pos+1 < big.len()-1-count {
                if big[pos-2] == '\n' {
                    big[pos+1] = '\\';
                    big[pos] = '0';
                    big[pos-1] = '\\';
                }
                big[pos-c] = w[times]; // Sets the big pos - c to wword times.
                let _ = big.iter().map(|x| { print!("{}", x); *x}).collect::<Vec<char>>();
                println!("");
                sleep(Duration::from_millis(100));
                Command::new("clear").status().unwrap();
                pos += 1;
                big[pos+1] = '|';
                big[pos] = '0';
                big[pos-1] = '|';
                big[pos-2] = ' ';
                big[pos-c] = w[times]; // Sets the big pos - c to wword times.
                let x = c+1;
                big[pos-x] = ' ';
                let _ = big.iter().map(|x| { print!("{}", x); *x}).collect::<Vec<char>>();
                println!("");
                sleep(Duration::from_millis(100));
                Command::new("clear").status().unwrap();
            }
            times -= 1;
            count += 1;
        }
    }
}
