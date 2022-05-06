use std::io;
fn main() {
    let mut z = true;
    let mut grid = String::from("[---------------------------------------------------------------------------------------]\n");
    for _ in 0..27 {
        grid.push_str("[.......................................................................................]\n");
    }
    grid.push_str("[---------------------------------------------------------------------------------------]");
    let mut pos = 112;
    let mut g = grid.chars().collect::<Vec<char>>();
    let mut inp = String::new();
    loop {
        io::stdin().read_line(&mut inp).unwrap();
        match inp.as_str() {
            "w\r\n" => {
                if g[pos-90] == '.' || g[pos-90] == '*' {
                    pos -= 90;
                    g[pos] = '|';
                    if !z {
                        g[pos+90] = '.';
                    }
                    z = false;
                }
            },
            "s\r\n" => {
                if g[pos+90] == '.' || g[pos+90] == '*' {
                    pos += 90;
                    g[pos] = '|';
                    if !z {
                        g[pos-90] = '.';
                    }
                    z = false;
                }
            },
            "a\r\n" => {
                if g[pos-1] == '.' || g[pos-1] == '*' {
                    pos -= 1;
                    g[pos] = '|';
                    if !z {
                        g[pos+1] = '.';
                    }
                    z = false;
                }
            },
            "d\r\n" => {
                if g[pos+1] == '.' || g[pos+1] == '*' {
                    pos += 1;
                    g[pos] = '|';
                    if !z {
                        g[pos-1] = '.';
                    }
                    z = false;
                }
            },
            "p\r\n" => {
                g[pos] = '*';
                z = true;
            }
            "e\r\n" => {
                g[pos] = '.';
                z = true;
            }
            "pup\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos-90;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '*';
                        p -= 90;
                    }
                }
            }
            "pdown\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos+90;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '*';
                        p += 90;
                    }
                }
            }
            "pleft\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos-1;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '*';
                        p -= 1;
                    }
                }
            }
            "pright\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos+1;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '*';
                        p += 1;
                    }
                }
            }
            "eup\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos-90;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '.';
                        p -= 90;
                    }
                }
            }
            "edown\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos+90;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '.';
                        p += 90;
                    }
                }
            }
            "eleft\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos-1;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '.';
                        p -= 1;
                    }
                }
            }
            "eright\r\n" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut p = pos+1;
                for _ in 0..input.trim().parse().unwrap() {
                    if g[p] == '.' || g[p] == '*' {
                        g[p] = '.';
                        p += 1;
                    }
                }
            }
            _ => {
            },
        }
        println!("{}", g.iter().collect::<String>());
        if inp != String::new() {
            inp = String::new();
        }
    }
}
