use std::thread::sleep;
use std::time::Duration;
use std::process::Command;
fn main() {
    let mut count0 = 0;
    let mut count1 = 0;
    let mut count2 = 0;
    let mut r = 0;
    let mut m = String::new();
    for _ in 0..4 {
        m.push_str(&" ".repeat(6));
        m.push_str(&"01".repeat(4));
        m.push_str(&" ".repeat(6));
        m.push('\n');
    }
    let mut mug: Vec<char> = m.chars().collect::<Vec<char>>();
    for i in 0..20 {
        for x in 0..20 {
            if i > 5 && x < 5 && x % 10 == 0 {
                for _ in 0..5 {
                    mug.push(' ');
                }
                mug.push('|');
            } else if i > 5 && x < 9 {
                if i != 19 {
                    mug.push('.');
                } else {
                    mug.push('_');
                }
            } else if i == 9 {
                if count0 == 0 {
                    mug.push('|');
                } else {
                    mug.push('-');
                }
                count0 += 1;
            } else if i == 7 {
                if count1 == 0 {
                    mug.push('|');
                } else {
                    mug.push('-');
                }
                count1 += 1;
            } else if i == 8 {
                if count2 < 2 {
                    mug.push('|');
                } else {
                    let len = mug.len()-1;
                    mug[len] = ' ';
                    mug.push('|');
                }
                count2 += 1;
            } else if i > 5 && x == 9 {
                mug.push('|');
            }
        }
        mug.push('\n');
        if r < 6 {
            let len = mug.len()-1;
            mug.remove(len);
        }
        r += 1;
    }
    let backup = mug.clone();
    let pos = (0..mug.len()-1).filter(|x| mug[*x] == ' ' && mug[x+1] == '0').collect::<Vec<usize>>();
    let mut count = 3;
    loop {
        println!("{}", mug.iter().collect::<String>());
        sleep(Duration::from_millis(250));
        Command::new("clear").status().unwrap();
        for i in 0..mug.len() {
            if pos.contains(&i) {
                if count < 4 {
                    for _ in 0..count {
                        mug.insert(i, ' ');
                    }
                }
                count -= 1;
            }
        }
        println!("{}", mug.iter().collect::<String>());
        sleep(Duration::from_millis(250));
        Command::new("clear").status().unwrap();
        mug = backup.clone();
        println!("{}", mug.iter().collect::<String>());
        sleep(Duration::from_millis(250));
        Command::new("clear").status().unwrap();
        count = 3;
        for i in 0..mug.len() {
            if pos.contains(&i) {
                if count < 4 {
                    for _ in 0..count {
                        mug.remove(i-5);
                    }
                }
                count -= 1;
            }
        }
        println!("{}", mug.iter().collect::<String>());
        sleep(Duration::from_millis(250));
        Command::new("clear").status().unwrap();
        count = 3;
        mug = backup.clone();
    }
}
