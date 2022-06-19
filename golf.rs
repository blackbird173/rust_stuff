use rand::Rng;
enum Sponsers {
   Apple,
   Mozilla,
   Google,
   Micorsoft,
   Github,
}
enum HowWell {
   VeryGood,
   Good,
   Mid,
   Bad,
   VeryBad,
}
struct Golfer {
   name: String,
   age: i32,
   score: i32,
   sponser: Sponsers,
}
impl Golfer {
   fn today_score(&self) -> (i32, HowWell) {
      let x: i32 = rand::thread_rng().gen_range(0..=4);
      let value: HowWell;
      match x {
         0 => {
            value = HowWell::VeryBad;
            (&self.score + 3, value)
         },
         1 => {
            value = HowWell::Bad;
            (&self.score + 1, value)
         },
         2 => {
            value = HowWell::Mid;
            (&self.score + 0, value)
         },
         3 => {
            value = HowWell::Good;
            (&self.score - 1, value)
         },
         4 => {
            value = HowWell::VeryGood;
            (&self.score - 3, value)
         },
         _ => {
            value = HowWell::Mid;
            (&self.score + 0, value)
         },
      }
   }
}
fn main() {
   let v = vec![Golfer{name: String::from("jim"), age: 19, score: 67, sponser: Sponsers::Micorsoft}, Golfer{name: String::from("bob"), age: 20, score: 65, sponser: Sponsers::Apple}, Golfer{name: String::from("joe"), age: 17, score: 69, sponser: Sponsers::Mozilla}, Golfer{name: String::from("rob"), age: 16, score: 65, sponser: Sponsers::Github}, Golfer{name: String::from("mark"), age: 16, score: 69, sponser: Sponsers::Google}];
   let scores = v.iter().map(|x| x.today_score()).collect::<Vec<(i32, HowWell)>>();
   let mut top_scores: Vec<(i32, String)> = vec![(scores[0].0, v[0].name.clone())];
   for i in 1..scores.len() {
      if scores[i].0 < top_scores[0].0 {
         top_scores = vec![];
         top_scores.push((scores[i].0, v[i].name.clone()));
      } else if scores[i].0 == top_scores[0].0 {
         top_scores.push((scores[i].0, v[i].name.clone()));
      }
   }
   println!("{:?}", top_scores);
}
