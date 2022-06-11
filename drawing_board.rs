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
    g[pos] = '|';
    loop {
        io::stdin().read_line(&mut inp).unwrap();
        match inp.as_str().trim() {
            "w" => {
                if g[pos-90] == '.' || g[pos-90] == '*' {
                    pos -= 90;
                    g[pos] = '|';
                    if !z {
                        g[pos+90] = '.';
                    }
                    z = false;
                }
            },
            "s" => {
                if g[pos+90] == '.' || g[pos+90] == '*' {
                    pos += 90;
                    g[pos] = '|';
                    if !z {
                        g[pos-90] = '.';
                    }
                    z = false;
                }
            },
            "a" => {
                if g[pos-1] == '.' || g[pos-1] == '*' {
                    pos -= 1;
                    g[pos] = '|';
                    if !z {
                        g[pos+1] = '.';
                    }
                    z = false;
                }
            },
            "d" => {
                if g[pos+1] == '.' || g[pos+1] == '*' {
                    pos += 1;
                    g[pos] = '|';
                    if !z {
                        g[pos-1] = '.';
                    }
                    z = false;
                }
            },
            "p" => {
                g[pos] = '*';
                z = true;
            }
            "e" => {
                g[pos] = '.';
                z = true;
            }
            "pup" => {
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
            "pdown" => {
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
            "pleft" => {
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
            "pright" => {
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
            "eup" => {
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
            "edown" => {
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
            "eleft" => {
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
            "eright" => {
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
