use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
use std::io::stdin;
fn main() {
    Command::new("clear").status().unwrap();
    println!("enter a word");
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let wword = " ".to_owned() + inp.trim();
    let mut count = 0;
    let mut times = wword.len()-1;
    let w = wword.chars().collect::<Vec<char>>();
    let mut big = String::from(wword.chars().map(|mut _x| { _x = ' '; _x }).collect::<String>() + "   " + "\n");
    for _ in 0..wword.len()+3 {
        big.push(' ');
    };
    let mut pos = big.len()-2;
    let mut big = big.chars().collect::<Vec<char>>();
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
            let _ = big.iter().map(|x| { print!("{}", x); *x}).collect::<Vec<char>>();
            println!("");
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
                big[pos-c] = w[times];
                let _ = big.iter().map(|x| { print!("{}", x); *x}).collect::<Vec<char>>();
                println!("");
                sleep(Duration::from_millis(100));
                Command::new("clear").status().unwrap();
                pos += 1;
                big[pos+1] = '|';
                big[pos] = '0';
                big[pos-1] = '|';
                big[pos-2] = ' ';
                big[pos-c] = w[times];
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
