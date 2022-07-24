use rand::Rng;
#[derive(Debug)]
#[derive(PartialEq)]
enum Scores {
    A,
    B,
    C,
    D,
    F
}
#[derive(Debug)]
#[derive(PartialEq)]
enum Grades {
    Q,
    W,
    E,
    R,
    T,
}
#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: u32,
}
impl Student<'_> {
    fn quiz(&self) -> Option<(&str, Grades, (Scores, i32))> {
        let mut grade = None;
        let mut score = None;
        match self.age {
            6..=7 => {
                grade = Some(Grades::Q);
            },
            8..=9 => {
                grade = Some(Grades::W);
            },
            10..=11 => {
                grade = Some(Grades::E);
            },
            12..=13 => {
                grade = Some(Grades::R);
            },
            14..=15 => {
                grade = Some(Grades::T);
            },
            _ => {},
        }
        match rand::thread_rng().gen_range(0..=100) {
            90..=100 => {
                score = Some((Scores::A, 5));
            },
            80..=89 => {
                score = Some((Scores::B, 4));
            },
            70..=79 => {
                score = Some((Scores::C, 3));
            },
            60..=69 => {
                score = Some((Scores::D, 2));
            },
            0..=59 => {
                score = Some((Scores::F, 1));
            },
            _ => {},
        }
        if grade != None && score != None {
            return Some((self.name, grade.unwrap(), score.unwrap()));
        }
        None
    }
}
fn main() {
    let students = [Student {name: "john", age: 6}, Student {name: "bob", age: 8}, Student {name: "jill", age: 13}, Student {name: "mark", age: 13}, Student {name: "macy", age: 14}];
    let results = students.iter().map(|x| {
        let quiz = x.quiz().unwrap();
        (x.name, quiz.2, quiz.1)
    }).collect::<Vec<(&str, (Scores, i32), Grades)>>();
    let current = *results.iter().map(|x| x.1.1).collect::<Vec<i32>>().iter().max().unwrap();
    for x in results.iter() {
        if current == x.1.1 {
            println!("name: {}, grade: {:?}, score: {:?}", x.0, x.2, x.1.0)
        }
    }
}
