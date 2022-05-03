use std::io;
use std::io::Write;
use std::fs::OpenOptions;
use std::time::Duration;
use std::thread;
fn main() {
    let you = String::from("|-___________________________________________________________");
    let mut ind = 1;
    let mut v = you.chars().collect::<Vec<char>>();
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let num: i32 = inp.trim().parse().unwrap();
    println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    for _ in 0..num as i32 {
        if ind < 60 {
            ind += 1;
            v[ind] = '-';
            v[ind-1] = '_';
        }
        thread::sleep(Duration::from_millis(100));
        println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        thread::sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H");
    }
    if ind < 60 {
        v[ind] = '|';
        v[ind-1] = '_';
    }
    thread::sleep(Duration::from_millis(100));
    println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    let mut f = OpenOptions::new().append(true).create(true).open("spear_logs.txt").unwrap();
    f.write_all("u threw the spear yay!\n".as_bytes()).unwrap();
    println!("u threw the spear {} feet", ind-1);
}
