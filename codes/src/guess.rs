use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let mut guess = String::new();
        println!("请输入您要猜的数字[1-10]:");
        io::stdin().read_line(&mut guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let secret = rand::thread_rng().gen_range(1, 11);
        println!("{}", secret);
        match guess.cmp(&secret) {
            Ordering::Greater => println!("猜大了，不要灰心哦~请再次输入您猜的数字"),
            Ordering::Less => println!("猜小了，不要灰心哦~请再次输入您猜的数字"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
