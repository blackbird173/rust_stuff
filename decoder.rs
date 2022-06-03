fn decoder(s: &str, s2: &str) -> String {
    let s2 = s2.chars().collect::<Vec<char>>();
    let mut count = 0;
    s.chars().collect::<Vec<char>>().iter().filter(|x| {
        if count < s2.len() && **x == s2[count] {
            count += 1;
            true
        } else {
            false
        }
    }).collect::<String>()
}
