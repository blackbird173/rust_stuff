use std::thread::sleep;
use std::time::Duration;
use std::io;
static HEAD: usize = 0;
static RIGHT_ARM: usize = 4;
static TORSO: usize = 5;
static LEFT_ARM: usize = 6;
static RIGHT_LEG: usize = 8;
static INVIS: usize = 9;
static LEFT_LEG: usize = 10;
static mut PERSON: &str = " 0 \n/|\\\n/ \\";
trait Stuff {
    fn walk(&self, steps: i32) {
        unsafe {
            let mut z = 0;
            let mut count = 0;
            let mut x = PERSON.chars().collect::<Vec<char>>();
            let mut space = String::new();
            for _ in 0..steps {
                let _ = x.iter().collect::<String>().chars().map(|i| {
                    if z == 0 || z == 4 || z == 8 {
                        print!("{}", space);
                    }
                    if i != '\n' || i != ' ' {
                        print!("{}", i);
                    }
                    z += 1;
                    i
                }).collect::<String>();
                if count%2 == 0 {
                    x[RIGHT_LEG] = '|';
                    x[LEFT_LEG] = '|';
                } else {
                    x[RIGHT_LEG] = '/';
                    x[LEFT_LEG] = '\\';
                }
                println!("");
                z = 0;
                count += 1;
                space.push(' ');
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
            }
        }
    }
    fn wave(&self, option: i32, times: i32) {
        unsafe {
            if option%2 == 0 {
                let mut x = PERSON.chars().collect::<Vec<char>>();
                x[RIGHT_ARM as usize] = '-';
                println!("{}", x.iter().collect::<String>());
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                for _ in 0..times {
                    x[0] = '\\';
                    x[RIGHT_ARM as usize] = ' ';
                    println!("{}", x.iter().collect::<String>());
                    x[0] = ' ';
                    sleep(Duration::from_millis(250));
                    std::process::Command::new("clear").status().unwrap();
                    x[0] = '|';
                    x[RIGHT_ARM as usize] = ' ';
                    println!("{}", x.iter().collect::<String>());
                    x[0] = ' ';
                    sleep(Duration::from_millis(250));
                    std::process::Command::new("clear").status().unwrap();
                }
                x[RIGHT_ARM as usize] = '/';
                println!("{}", x.iter().collect::<String>());
                sleep(Duration::from_millis(250));
            std::process::Command::new("clear").status().unwrap();
            } else {
                let mut x = PERSON.chars().collect::<Vec<char>>();
                x[LEFT_ARM as usize] = '-';
                println!("{}", x.iter().collect::<String>());
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                for _ in 0..times {
                    x[2] = '/';
                    x[LEFT_ARM as usize] = ' ';
                    println!("{}", x.iter().collect::<String>());
                    x[2] = ' ';
                    sleep(Duration::from_millis(250));
                    std::process::Command::new("clear").status().unwrap();
                    x[2] = '|';
                    x[LEFT_ARM as usize] = ' ';
                    println!("{}", x.iter().collect::<String>());
                    x[2] = ' ';
                    sleep(Duration::from_millis(250));
                    std::process::Command::new("clear").status().unwrap();
                }
                x[LEFT_ARM as usize] = '\\';
                println!("{}", x.iter().collect::<String>());
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
            }
        }
    }
    fn flip(&self, times: i32) {
        unsafe {
            for _ in 0..times {
                println!("{}", PERSON);
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                let f = " \\/\n --0\n /\\";
                println!("{}", f);
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                let f = "\\ /\n |\n|0|";
                println!("{}", f);
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                let f = "  \\/ \n 0--\n  /\\";
                println!("{}", f);
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
                println!("{}", PERSON);
                sleep(Duration::from_millis(250));
                std::process::Command::new("clear").status().unwrap();
            }
        }
    }
}
impl Stuff for &str {}
fn main() {
    let mut inp = String::new();
    unsafe {
        loop {
            io::stdin().read_line(&mut inp).unwrap();
            match inp.trim() {
                "wave" => {
                    println!("times?");
                    let mut times = String::new();
                    io::stdin().read_line(&mut times).unwrap();
                    println!("left or right (even for right odd for left)?");
                    let mut op = String::new();
                    io::stdin().read_line(&mut op).unwrap();
                    PERSON.wave(op.trim().parse::<i32>().unwrap(), times.trim().parse::<i32>().unwrap());
                },
                "flip" => {
                    println!("times?");
                    let mut times = String::new();
                    io::stdin().read_line(&mut times).unwrap();
                    PERSON.flip(times.trim().parse::<i32>().unwrap());
                },
                "walk" => {
                    println!("times?");
                    let mut times = String::new();
                    io::stdin().read_line(&mut times).unwrap();
                    PERSON.walk(times.trim().parse::<i32>().unwrap());
                },
                _ => {},
            }
            inp.clear();
        }
    }
}
