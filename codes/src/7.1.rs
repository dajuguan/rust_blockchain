#[derive(Debug)]
struct square<T> {
    x: T,
}

impl<T1> square<T1> {
    fn item(&self) -> &T1 {
        return &self.x;
    }
}

fn main() {
    let s = square { x: 1 };
    println!("{}", s.item())
}
