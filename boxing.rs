use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, Write};
use std::time::Duration;
use std::fs::OpenOptions;
use rand::Rng;
fn main() {
    let mut f = OpenOptions::new().append(true).create(true).open("boxing.txt").unwrap();
    let mut _uhealth = 3;
    let ehealth = Arc::new(Mutex::new(3));
    let inp = Arc::new(Mutex::new(String::new()));
    let mut ring = String::from("[-----------]\n");
    for _ in 0..5 {
        ring += "[...........]\n";
    }
    ring += "[-----------]";
    let v: Arc<Mutex<Vec<char>>> = Arc::new(Mutex::new(ring.chars().collect()));
    let pos = Arc::new(Mutex::new(16));
    let pos2 = Arc::new(Mutex::new(v.lock().unwrap().len()-19));
    v.lock().unwrap()[*pos.lock().unwrap()] = '|';
    v.lock().unwrap()[*pos2.lock().unwrap()] = '}';
    loop {
        {
            let inp = inp.clone();
            let v = v.clone();
            let pos = pos.clone();
            let ehealth = ehealth.clone();
            thread::spawn(move || {
                io::stdin().read_line(&mut inp.lock().unwrap()).unwrap();
                let x: &str = &*inp.lock().unwrap().clone();
                match x.trim() {
                    "w" => {
                        if v.lock().unwrap()[*pos.lock().unwrap()-14] == '.' {
                            *pos.lock().unwrap() -= 14;
                            v.lock().unwrap()[*pos.lock().unwrap()] = '|'; 
                            v.lock().unwrap()[*pos.lock().unwrap()+14] = '.'; 
                        }
                    },
                    "s" => {
                        if v.lock().unwrap()[*pos.lock().unwrap()+14] == '.' {
                            *pos.lock().unwrap() += 14;
                            v.lock().unwrap()[*pos.lock().unwrap()] = '|'; 
                            v.lock().unwrap()[*pos.lock().unwrap()-14] = '.'; 
                        }
                    },
                    "a" => {
                        if v.lock().unwrap()[*pos.lock().unwrap()-1] == '.' {
                            *pos.lock().unwrap() -= 1;
                            v.lock().unwrap()[*pos.lock().unwrap()] = '|'; 
                            v.lock().unwrap()[*pos.lock().unwrap()+1] = '.'; 
                        }
                    },
                    "d" => {
                        if v.lock().unwrap()[*pos.lock().unwrap()+1] == '.' {
                            *pos.lock().unwrap() += 1;
                            v.lock().unwrap()[*pos.lock().unwrap()] = '|'; 
                            v.lock().unwrap()[*pos.lock().unwrap()-1] = '.'; 
                        }
                    },
                    "f" => {
                        let _one = v.lock().unwrap()[*pos.lock().unwrap()-14];
                        let _two = v.lock().unwrap()[*pos.lock().unwrap()+14];
                        let _three = v.lock().unwrap()[*pos.lock().unwrap()-1];
                        let _four = v.lock().unwrap()[*pos.lock().unwrap()+1];
                        if _one == '}' || _two == '}' || _three == '}' || _four == '}' {
                            *ehealth.lock().unwrap() -= 1;
                        }
                    },
                    _ => {},
                }
                if *inp.lock().unwrap() != String::new() {
                    *inp.lock().unwrap() = String::new();
                }
            });
        }
        if *ehealth.lock().unwrap() == 0 {
            f.write_all("u win\n".as_bytes()).unwrap();
            break;
        }
        if _uhealth == 0 {
            f.write_all("u lose\n".as_bytes()).unwrap();
            break;
        }
        let mut x = rand::thread_rng();
        match x.gen_range(0..=4) {
            0 => {
                if v.lock().unwrap()[*pos2.lock().unwrap()-14] == '.' {
                    *pos2.lock().unwrap() -= 14;
                    v.lock().unwrap()[*pos2.lock().unwrap()] = '}'; 
                    v.lock().unwrap()[*pos2.lock().unwrap()+14] = '.'; 
                }
            },
            1 => {
                if v.lock().unwrap()[*pos2.lock().unwrap()+14] == '.' {
                    *pos2.lock().unwrap() += 14;
                    v.lock().unwrap()[*pos2.lock().unwrap()] = '}'; 
                    v.lock().unwrap()[*pos2.lock().unwrap()-14] = '.'; 
                }
            },
            2 => {
                if v.lock().unwrap()[*pos2.lock().unwrap()-1] == '.' {
                    *pos2.lock().unwrap() -= 1;
                    v.lock().unwrap()[*pos2.lock().unwrap()] = '}'; 
                    v.lock().unwrap()[*pos2.lock().unwrap()+1] = '.'; 
                }
            },
            3 => {
                if v.lock().unwrap()[*pos2.lock().unwrap()+1] == '.' {
                    *pos2.lock().unwrap() += 1;
                    v.lock().unwrap()[*pos2.lock().unwrap()] = '}'; 
                    v.lock().unwrap()[*pos2.lock().unwrap()-1] = '.'; 
                }
            },
            4 => {
                let _one = v.lock().unwrap()[*pos2.lock().unwrap()-14];
                let _two = v.lock().unwrap()[*pos2.lock().unwrap()+14];
                let _three = v.lock().unwrap()[*pos2.lock().unwrap()-1];
                let _four = v.lock().unwrap()[*pos2.lock().unwrap()+1];
                if _one == '|' || _two == '|' || _three == '|' || _four == '|' {
                    _uhealth -= 1;
                }
            }, _ => {}
        }
        println!("ur health: {}, enemey health: {}", _uhealth, ehealth.lock().unwrap());
        println!("{}", v.lock().unwrap().iter().collect::<String>());
        thread::sleep(Duration::from_millis(200));
        print!("\x1B[2J\x1B[1;1H");
    }
}
