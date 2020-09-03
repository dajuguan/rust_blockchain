# 5 结构体和方法

## 5 结构体\(struct\)与方法

## 结构体

结构体类似于面向对象语言中的**属性**。同时结构体也与第1章介绍的元组类似，都可以包含不同的数据。但是不同的地方时，结构体可以使用属性来定义变量，这样就不需要索引来访问了，同时还可以通过后面的impl语句给结构体定义相应的方法。

### 定义与实例化结构体

定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）。例如:

```text
#[derive(Debug)]
struct Student {
    name: String,
    score: f64,
    grade: u64,
    in_school: bool,
}

fn main() {
    let s = Student {
        name: String::from("chen"),
        score: 12.0,
        grade: 2,
        in_school: true,
    };
    println!("{:?}", s);
}
```

需要注意的是我们必须为每一个字段初始化，rust不会像其他语言那样赋值默认值。同时这里，我们使用了Debug这个trait来打印Student，因为Student默认没有绑定对应的打印方法。

## struct初始化简写方法

如果初始化的变量和struct的字段名一样的话，就可以不写字段名字了:

```text
#[derive(Debug)]
struct Student {
    name: String,
    score: f64,
    grade: u64,
    in_school: bool,
}

fn new_student(name: String, score: f64) -> Student {
    Student {
        name,
        score,
        grade: 1,
        in_school: true,
    }
}

fn main() {
    let s = new_student(String::from("chen"), 99.0);
    println!("{:?}", s);
}
```

## 结构体更新语法

使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有帮助的。这可以通过 结构体更新语法（struct update syntax）实现。

下面的例子，展示我们如何用s的值去，设置s1中未显式设置的grade和in\_school字段:

```text
#[derive(Debug)]
struct Student {
    name: String,
    score: f64,
    grade: u64,
    in_school: bool,
}
fn main() {
    let s = Student {
        name: String::from("Alice"),
        score: 99.0,
        grade: 1,
        in_school: true,
    };
    let s1 = Student {
        name: String::from("Bob"),
        score: 89.0,
        ..s
    };
    println!("{:?}", s1);
}
```

## 元组结构体

当我们想给结构体命名时，也可以使用struct，只不过元组结构体没有结构体的字段名:

```text
struct Color(f32, f32, f32, f32);

fn main() {
    let c = Color(1.0, 1.0, 1.0, 1.0);
    println!("{}", c.1);
}
```

> 结构体数据的所有权

```text
在示例 5-1 中的 User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。

可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes），这是一个第十章会讨论的 Rust 功能。生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：

文件名: src/main.rs
```
[这些代码不能编译！] 
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

编译器会抱怨它需要生命周期标识符：
```
error[E0106]: missing lifetime specifier
-->
|
2 |     username: &str,
|               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
-->
|
3 |     email: &str,
|            ^ expected lifetime parameter
```
在后面会讲到如何修复这个问题以便在结构体中存储引用，不过现在，我们会使用像 String 这类拥有所有权的类型来替代 &str 这样的引用以修正这个错误。
```

## 方法

方法与函数类似，不过它是定义在结构体上，并且它的第一个参数总是**结构体实例的引用:&self**。

### 方法定义

```text
struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    pub fn area(&self) -> f64 {
        &self.width * &self.height
    }
}

fn main() {
    let r = Rect {
        width: 10.1,
        height: 10.2,
    };
    println!("{}", r.area());
}
```

如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。

### 关联函数

**impl**还有一个功能是，如果第一个参数**不**传入self，那么他就是**关联函数\(associated functions\)**.它们不作用于一个结构体实例，仍然是函数. 关联函数经常被用作返回一个结构体新实例的构造函数。 采用**结构体名::函数名**这样的方式调用:

```text
#[derive(Debug)]
struct Rect {
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Rect {
        Rect { width, height }
    }
}

fn main() {
    let r = Rect::new(10.0, 11.0);
    println!("{:?}", r);
}
```

## 总结

* 通过结构体将不同的属性组合到一起
* 通过方法定义了结构体的行为

