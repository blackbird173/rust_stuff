fn to_binary(mut num: u32) -> String {
    let mut i: u32 = 0;
    while num > 2u32.pow(i) {
        i += 1;
    }
    (0..i).rev().map(|x| {
        if num >= 2u32.pow(x) {
            num -= 2u32.pow(x);
            return "1";
        } else {
            return "0";
        }
    }).collect::<String>()
}
fn from_binary(s: &str) -> u32 {
    let mut ss = s.chars().collect::<Vec<char>>();
    let mut num = 0;
    ss.reverse();
    for i in 0..ss.len() {
        num += ss[i].to_string().parse::<u32>().unwrap() * 2u32.pow(i as u32);
    }
    num
}
fn main() {
    println!("{}", to_binary(1273));
    println!("{}", from_binary(to_binary(1273).as_str()));
}
