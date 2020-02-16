enum Direction {
    Right,
    Left,
}

fn main() {
    let d = Direction::Right;
    let s = Direction::Left;
    match d {
        (s) => return,
        Direction::Left => println!("Left"),
        _ => println!("other"),
    }
}
