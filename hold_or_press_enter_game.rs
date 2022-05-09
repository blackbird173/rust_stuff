use std::fs::{OpenOptions, File};
use std::io::{self, Read};
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
fn main() {
    let mut high_score = OpenOptions::new().append(true).create(true).read(true).open("logs.txt").unwrap();
    let time = Arc::new(Mutex::new(30));
    let input = Arc::new(Mutex::new(String::new()));
    let presses = Arc::new(Mutex::new(0));
    println!("lets see how many times u can press (or u can just hold down) enter");
    loop {
        {
        let input = input.clone();
        let presses = presses.clone();
        thread::spawn(move || {
            io::stdin().read_line(&mut input.lock().unwrap()).unwrap();
            if *input.lock().unwrap() != String::new() {
                *input.lock().unwrap() = String::new();
                *presses.lock().unwrap() += 1;
            }
            println!("times hit {}", presses.lock().unwrap());
        });
        let time = time.clone();
        thread::spawn(move || {
            *time.lock().unwrap() -= 1;
        });
        }
        println!("time left {}", *time.lock().unwrap());
        if *time.lock().unwrap() == 0 {
            high_score.write_all(presses.lock().unwrap().to_string().as_bytes()).unwrap();
            high_score.flush().unwrap();
            high_score.write_all("\n".as_bytes()).unwrap();
            high_score.flush().unwrap();
            let mut f = File::open("logs.txt").unwrap();
            let mut con = String::new();
            f.read_to_string(&mut con).unwrap();
            let v: Vec<String> = con.split("\n").map(|x| x.to_string()).collect();
            let x = &mut v[0].parse::<i32>().unwrap();
            for i in v {
                if i != "" {
                    if i.parse::<i32>().unwrap() >= *x {
                        *x = i.parse::<i32>().unwrap();
                    }
                }
            }
            println!("ur high score is {}", x);
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
}
