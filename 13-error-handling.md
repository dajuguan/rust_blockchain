# 13 错误处理

# panic

panic 是不可恢复的错误，它打印错误堆栈，并直接退出程序.

```
fn test_panic() {
    panic!("错误!");
    println!("未执行",)
}

fn main() {
    test_panic();
}

```

# Option&wrap

Option 可以用来处理有值和无值的情况；也可以直接要陪你过 unwrap 来获取 option 的值，但是如果无值就会引发 panic.

```
fn give_commoner(gift: Option<&str>) {
    // Specify a course of action for each case.
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No gift? Oh well."),
    }
}

// Our sheltered princess will `panic` at the sight of snakes.
// All gifts are handled implicitly using `unwrap`.
fn give_princess(gift: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let food  = Some("cabbage");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}
```

## 使用？来解构 Option

更方便的方式去解构 Option 是采用？而不是 match、如果 x 是一个 Option，那么如果 x 是 Some 的话 x?将会返回 Some 里的值；否则，后面的语句都不会执行，直接返回 None。

```
fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If `current_age` is `None`, this returns `None`.
    // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    let next_age: u8 = current_age?;
    println!("excuted");
    Some(format!("Next year I will be {}", next_age))
}

fn main() {
    let age = Some(2u8);
    let res = next_birthday(age);
    println!("{:?}", res);
    let age = None;
    next_birthday(age);  //next_birthday函数中后面的语句了
}

```
