#[derive(Debug)]
enum Wrapper<T> {
    Value(T),
}
impl<T> Wrapper<T> {
    fn set_value(&mut self, x: T) {
        *self = Wrapper::Value(x);
    }
    fn get_value(&self) -> &T {
        match &self {
            Self::Value(z) => {
                z
            },
        }
    }                                                   
}
fn main() {
    let mut x = Wrapper::Value("hello world");
    x.set_value("goodbye world");
    println!("{}", x.get_value());
}
