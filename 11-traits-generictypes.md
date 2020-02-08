# 11 泛型(generics)、trait 和生命周期

和 C++的面向对象程序设计语言一样，rust 也支持泛型，实现不事先指定类型，实现不同数据类型的高效复用。
然后通过 trait 来实现接口，为泛型限定其行为（也就是操作或者函数);
最后 rust 的生命周期允许我们引用相互关联的泛型，并且编译器能在引用很多值的时候依然检查其有效性。

# 泛型

## 定义

在结构体或函数的后面声明<T>，就可以在内部使用 T 类型了。

```
#[derive(Debug)]
struct square<T> {
    x: T,
}

fn main() {
    let s = square { x: 1 };
    println!("{:?}", s);
    let s = square { x: "a" };
    println!("{:?}", s);
}

```

对泛型加上方法:

```
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
```

需要注意的是，方法后面需要加上泛型,比如<T1>,否则就只会是具体类型的方法，比如

```
use std::fmt::Debug;
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Debug> Point<T> {
    pub fn display(&self) {
        println!("x:{:?},y:{:?}", &self.x, &self.y);
    }
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let s = Point { x: "a", y: "b" };
    s.display();
    // println!("distance:{}", s.distance_from_origin());  //编译不通过
    let ss = Point { x: 3.0, y: 4.0 };
    ss.display();
    println!("distance:{}", ss.distance_from_origin());
}
```

s 编译时无法使用 distance_from_origin 的方法，因为该方法只对 f32 类型有效。

## 多个泛型

可以使用多种不同类型的泛型，比如：

```
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let s = Point { x: 1, y: 'a' };
    println!("{:?}", s);
}
```

## 泛型代码性能

实际上编译的过程中，Rust 通过在编译时进行泛型代码的单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。这样既减少了程序猿的代码量，又实现了程序运行的时候不存在任何性能损失！就像程序为我们手写了重复的代码，太爽了有木有。

# trait

## 定义

一个类型的行为由其方法(impl)来定义，但是当不同的类型有相同的方法时，就可以通过 trait 来共享相同的行为，trait 可以理解为 C++或者 Go 中的接口。
接口可以定义一个或者一组行为（用分号隔开）。
接口中的方法前面不需要加 pub 关键字，因为它是需要在具体的结构体中实现的。

## 实现 Iterator 的 trait 来构建斐波那契数列

```
struct Fib {
    c: i32,
    n: i32,
}

impl Iterator for Fib {
    // add code here
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;
        Some(self.c)
    }
}

fn main() {
    let f = Fib { c: 1, n: 1 };
    for j in f.skip(14).take(10) {
        println!("{}", j);
    }
}

```

```
cargo run
```

## trait 作为参数

类似于基本类型作为参数一样，只不过是可以接受实现了该 trait 的参数

```
pub trait Area {
    fn area(&self) -> f64;
}

pub struct Rect {
    x: f64,
    y: f64,
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return &self.x * &self.y;
    }
}

fn getArea(item: impl Area) {
    println!("{}", item.area())
}

fn main() {
    let a = Rect { x: 1.2, y: 1.2 };
    getArea(a);
}
```

## 多个 trait 与 trait bound 语法糖

### 通过函数声明后增加 trait 类型，来限定参数的行为，成为 traitbound：

```
pub fn notify<T: Summary>(item1: T, item2: T) {
```

### 通过+指定多个 trait bound:

```
pub fn notify<T: Summary + Display>(item: T) {
```

### where 语句简化 trait bound

然而，使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。所以除了这么写：

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

还可以像这样使用 where 从句：

```
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

# 生命周期

在 Rust 中为了避免产生悬挂引用，所有变量的引用都会有自己的作用域，而在 rust 中这个作用域可以像泛型一样被显式的声明出来，就叫做引用的生命周期。它定义了参数引用和返回值引用之间作用域的大小关系，比如：

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这表明函数的返回值必须与 x 和 y 的作用域重叠，也就是求最小的作用域；如果不定义的话，那么可能会导致在不同的作用域引用一个已经不存在的值，产生悬挂引用,这在 C++等中很常见，但是 Rust 在编译阶段就会不通过！

需要注意的是，生命周期并不是一个固定值，在函数、或者结构体中，它是会把相应变量的作用域带入进去，然后如果所有引用变量的生命周期关系声明的是一样的话，那么就取最小的，比如:

```
fn max<'a>(x: &'a Point, y: &'a Point) -> &'a Point {
    return y;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let x = Point { x: 1, y: 2 };
    let y;
    {
        let c = Point { x: 2, y: 3 };
        y = max(&x, &c);
    }
    println!("{:?}", y);
}
```

```
  --> src/main.rs:15:21
   |
15 |         y = max(&x, &c);
   |                     ^^ borrowed value does not live long enough
16 |     }
   |     - `c` dropped here while still borrowed
17 |     println!("{:?}", y);
   |                      - borrow later used here
```

这里的'a，在编译时代入的实际上是 c 的作用域和 x 的作用域求交，也就是 c 的作用域，因此编译不会通过.

## 声明周期省略

## 静态声明周期
