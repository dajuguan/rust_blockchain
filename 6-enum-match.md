# 6 枚举(enums)与模式匹配(match)

# 枚举

本章介绍 **枚举（enumerations**），也被称作 _enums_。枚举允许你通过列举可能的 **成员（variants）** 来定义一个类型。
枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的 代数数据类型（algebraic data types）最为相似。

## 枚举定义

enum 关键字后面再接上成员的名字,比如 Ip 地址分为 Ipv4 和 Ipv6:

```
enum IpAddr {
    V4,
    V6,
}
```

## 枚举值

如果想让枚举包含有意义的值，可以很方便的实现;

```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let my_v4_addr = IpAddr::V4(127, 0, 0, 2);
    let my_v6_addr = IpAddr::V6(String::from("::2"));
}

```

可以通过 use 语句来进行简写:

```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

use crate::IpAddr::{V4, V6};

fn main() {
    let my_v4_addr = V4(127, 0, 0, 2);
    let my_v6_addr = V6(String::from("::2"));
}
```

## Option 枚举 Vs 空值

在很多语言中，都有空值(null)的概念，变量要么有值，要么是空值。

Tony Hoare，null 的发明者，在他 2009 年的演讲 “Null References: The Billion Dollar Mistake” 中曾经说到：

> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn't resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
>
> 我称之为我十亿美元的错误。当时，我在为一个面向对象语言设计第一个综合性的面向引用的类型系统。我的目标是通过编译器的自动检查来保证所有引用的使用都应该是绝对安全的。不过我未能抵抗住引入一个空引用的诱惑，仅仅是因为它是这么的容易实现。这引发了无数错误、漏洞和系统崩溃，在之后的四十多年中造成了数十亿美元的苦痛和伤害。

当你像非空值一样使用空值，就会导致难以预期的 bug；但是空值仍然有意义，它可以表示无效或者缺失。

为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 **Option<T>**，而且它定义于标准库中，如下:

```
enum Option<T> {
    Some(T),
    None,
}
```

Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。

那么 Option 相对于空值的优势在哪里呢？就是如果使用非 Option 的值，你可以**安全**地认为它一定有值；如果想同时处理空值和有值的情况，就必须显式地放入 Option 中，并处理每一种情况。
_这样麻麻就再也不用担心我们会错误地假设一个非空值！_

那么我们改如何处理 Option 的值呢?match 应运而生。

# match

match 是 Rust 中极为强大的控制流运算符，不同于其他语言的 switch 只能处理有限的情况，match 可以使用模式匹配强大的表现力，来对所有的模式进行处理。

```
fn main() {
    let cond = true;
    match cond {
        true => println!("true"),
        false => println!("false"),
    }
}

```

结合 enum，可以发挥强大的效力：

```
enum Weather {
    Sunny,
    Cludy,
}

fn weather_str(w: Weather) {
    match w {
        Weather::Sunny => println!("It's sunny"),
        Weather::Cludy => println!("It's Cloudy"),
    }
}
fn main() {
    let w = Weather::Sunny;
    weather_str(w);
}
```

# 绑定值

match 的一个作用是，可以把匹配到模式中的部分值绑定，这样可以实现提取枚举成员的值。
比如上面的例子，我们希望知道阴天 Cloudy 是周几，就在上面绑定了周几的 enum，然后就可以读取出来:

```
#[derive(Debug)]
enum Week {
    Monday,
    Friday,
}
enum Weather {
    Sunny,
    Cloudy(Week),
}

use crate::Weather::*;

fn weather_str(w: Weather) {
    match w {
        Sunny => println!("It's sunny"),
        Cloudy(week) => {
            println!("State From :{:?}", week);
        }
    }
}
fn main() {
    let w = Cloudy(Week::Monday);
    weather_str(w);
}
```

## 匹配 Option

我们在之前的部分中使用 Option<T> 时，是为了从 Some 中取出其内部的 T 值；我们还可以像处理 Weather 枚举那样使用 match 处理 Option<T>！

比如我们想要编写一个函数，它获取一个 Option<i32> 并且如果其中有一个值，将其加一。如果其中没有值，函数应该返回 None 值并不尝试执行任何操作。

得益于 match，编写这个函数非常简单，它将看起来像示例中这样：

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

```

## \_通配符

Rust 也提供了默认匹配的通配符 \_ ,类似于其他语言中的 default。

```
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```

## match 总结

- match 匹配是穷尽的，必须匹配每一种情况
- 可以通过 match 获取绑定的值

但是如果只处理一种情况 match 显得过于冗长了，这时我们可以使用 if let 语句:

# if let 语句

比如说，我想处理一个星期天就睡觉，其他时间上班的语句，使用 match，和 if let 分别比较一下

```
enum Week {
    WorkDay,
    Weekend,
}

fn main() {
    let day = Week::Weekend;
    match day {
        Week::Weekend => println!("休息"),
        _ => println!("干活"),
    }
    if let Week::Weekend = day {
        println!("休息")
    } else {
        println!("干活")
    }
}
```

可见 _if let_ 会相对简洁一些。

# 总结

- 通过枚举可以创建一些列可列举的类型
- match 和 Option 确保你的函数只会得到它期望的类型的值
- 枚举情况较少时可以使用**if let**来简化 match
