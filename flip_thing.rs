use std::{thread, sync::{Arc, Mutex}, io, cell::RefCell, time::Duration, fs::OpenOptions, io::Write};
use rand::Rng;
fn main() {
    let f = RefCell::new(OpenOptions::new().create(true).append(true).open("flip_logs.txt").unwrap());
    let inp = Arc::new(Mutex::new(String::new()));
    let time = RefCell::new(30);
    let times_flipped = Arc::new(Mutex::new(0));
    let times_missed = Arc::new(Mutex::new(0));
    while *time.borrow_mut() > 0 {
        {
            let inp = inp.clone();
            let times_flipped = times_flipped.clone();
            let times_missed = times_missed.clone();
            thread::spawn(move || {
                io::stdin().read_line(&mut inp.lock().unwrap()).unwrap();
                if *inp.lock().unwrap() != String::new() {
                    let rand: i32 = rand::thread_rng().gen_range(1..=4);
                    match rand {
                        1 => {
                            println!("u landed it YAY");
                            *times_flipped.lock().unwrap() += 1;
                        },
                        2 => {
                            println!("u did not land it");
                            *times_missed.lock().unwrap() += 1;
                        },
                        3 => {
                            println!("u did not land it");
                            *times_missed.lock().unwrap() += 1;
                        },
                        4 => {
                            println!("u landed it YAY");
                            *times_flipped.lock().unwrap() += 1;
                        },
                        _ => {},
                    }
                }
                *inp.lock().unwrap() = String::new();
            });
        }
        *time.borrow_mut() -= 1;
        thread::sleep(Duration::from_millis(1000));
        println!("time left: {}", time.borrow_mut());
    }
    f.borrow_mut().write_all("times flipped: ".as_bytes()).unwrap();
    f.borrow_mut().write_all(times_flipped.lock().unwrap().to_string().as_bytes()).unwrap();
    f.borrow_mut().write_all("\n".as_bytes()).unwrap();
    f.borrow_mut().write_all("times missed: ".as_bytes()).unwrap();
    f.borrow_mut().write_all(times_missed.lock().unwrap().to_string().as_bytes()).unwrap();
    f.borrow_mut().write_all("\n".as_bytes()).unwrap();
}
