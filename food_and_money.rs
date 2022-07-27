enum Food {
    Pasta(u32, u32),
    Rice(u32, u32),
    Beef(u32, u32),
    Pork(u32, u32),
    Cheese(u32, u32),
    Water(u32, u32),
    Bread(u32, u32),
    Milk(u32, u32),
}
fn get_varient(f: &Food) -> (&u32, &u32) {
    match f {
        Food::Pasta(amount, cost) => (&amount, &cost),
        Food::Rice(amount, cost) => (&amount, &cost),
        Food::Beef(amount, cost) => (&amount, &cost),
        Food::Pork(amount, cost) => (&amount, &cost),
        Food::Cheese(amount, cost) => (&amount, &cost),
        Food::Water(amount, cost) => (&amount, &cost),
        Food::Bread(amount, cost) => (&amount, &cost),
        Food::Milk(amount, cost) => (&amount, &cost),
    }
}
struct Person<'a> {
    name: &'a str,
    money: u32,
    food_list: Vec<Food>
}
impl Person<'_> {
    fn buy(&self) -> (bool, u32) {
        let end = self.food_list.iter().fold(self.money, |money, food| money-get_varient(food).0*(get_varient(food).1));
        if end > 0 {
            (true, end)
        } else {
            (false, 0)
        }
    }
}
fn main() {
    let dude = Person {name: "dude", money: 100, food_list: vec![Food::Pasta(1, 5), Food::Cheese(1, 3), Food::Water(1, 1), Food::Bread(1, 2), Food::Milk(1, 7)]};
    println!("{} bought some food and has {} dollars left", dude.name, dude.buy().1);
}
