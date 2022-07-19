use rand::Rng;
struct Person<'a> {
    name: &'a str,
    money: i32,
}
#[derive(Debug)]
enum Colors {
    Green,
    Blue,
    Red,
    Yellow,
    Purple,
}
#[derive(Debug)]
enum Prizes {
    FrogToy(Colors),
    Car,
    Fan,
    Computer,
}
fn main() {
    let crates = (0..101).collect::<Vec<i32>>();
    let winning = crates[rand::thread_rng().gen_range(0..crates.len())];
    let mut dude = Person{name: "bob", money: 0};
    let mut prize: Vec<Option<Prizes>> = vec![];
    let mut won = false;
    let mut gamble = || -> (Option<Prizes>, bool) {
        let random_create  = crates[rand::thread_rng().gen_range(0..crates.len())];
        if random_create == winning {
            won = true;
        }
        match random_create {
            0..=50 => {
                dude.money -= random_create;
            },
            0..=100 => {
                dude.money += random_create;
            },
            _ => {},
        }
        match dude.money {
            0..=100 => {
                (Some(Prizes::FrogToy(Colors::Green)), won)
            },
            101..=200 => {
                (Some(Prizes::FrogToy(Colors::Blue)), won)
            },
            201..=300 => {
                (Some(Prizes::FrogToy(Colors::Red)), won)
            }
            301..=400 => {
                (Some(Prizes::FrogToy(Colors::Yellow)), won)
            },
            401..=500 => {
                (Some(Prizes::FrogToy(Colors::Purple)), won)
            },
            501..=600 => {
                (Some(Prizes::Car), won)
            },
            601..=700 => {
                (Some(Prizes::Fan), won)
            },
            701.. => {
                (Some(Prizes::Computer), won)
            },
            _ => {
                (None, won)
            },
        }
    };
    loop {
        let stuff = gamble();
        if stuff.1 {
            println!("{} won {} dollars and these prizes {:?}", dude.name, dude.money, prize);
            break;
        }
        prize.push(stuff.0);
    }
}
