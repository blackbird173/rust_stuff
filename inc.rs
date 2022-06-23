fn inc(v: &str) -> Vec<&str> {
   let mut new: Vec<&str> = vec![];
   let mut count = v.len();
   for i in 0..v.len() {
      new.push(&v[0..i+1]);
   }
   for _ in (0..v.len()).rev() {
      new.push(&v[0..count]);
      count -= 1;
   }
   new
}
fn main() {
   let _ = inc("hello world").into_iter().map(|x| {
      println!("{}", x);
      x
   }).collect::<Vec<&str>>();
}
