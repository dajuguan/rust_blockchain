# 4 引用与借用&Slice

## 4 引用与借用

上一节我们讲到如果想在函数中获取一个值，同时不改变其所有权的方式是引用。通过在变量前面加一个标识符&，就可以实现引用。我们举个简单的例子来看下：

文件名: src/main.rs

```text
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

可以这么理解，它是如果不move所有权的: ![&#x5F15;&#x7528;](../.gitbook/assets/4-ref.svg)

_&s1_ 语法让我们创建一个 **指向** 值 _s1_ 的引用，但是并不拥有它。因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。

我们将获取引用作为函数参数称为 借用（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。

那么我们能否修改引用的值吗？

```text
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.push_str("he");
    return s.len();
}
```

运行一下，发现不可以，这是因为引用的值**默认是不可变**的。

## 可变引用

通过把变量、传递的参数和函数签名加上_mut_关键字，来实现可变引用:

```text
fn main() {
    let mut s = String::from("hello");
    change_str(&mut s);
}

fn change_str(s: &mut String) {
    s.push_str(",world!");
}
```

需要注意的是，rust只允许同时又一个变量拥有可变引用，这是为了避免数据竞争.数据竞争一般由一下条件产生:

* 两个以上指针同时读数据
* 至少一个指针写数据
* 数据没有同步访问机制

下面这样的语句，不会通过

```text
fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    let s2 = &mut s;
    println!("{} {}", s1, s2)
}
```

类似的，也不能对一个变量，同时又可变引用和不可变引用;

```text
fn main() {
    let mut s = String::from("hello");
    let s1 = &s; // 没问题
    let s2 = &s; // 没问题
    let s3 = &mut s; // 大问题
    println!("{} {} {}", s1, s2, s3);
}
```

因为我们不希望一个不可变的引用在使用的时候，被其他使用者改变了！

通过修改一下，改变变量的作用域才可以:

```text
fn main() {
    let mut s = String::from("hello");
    let s1 = &s; // 没问题
    let s2 = &s; // 没问题
    println!("{} {}", s1, s2);
    let s3 = &mut s; // 大问题
    println!("{}", s3);
}
```

因为s3引用s的时候，s1和s2的生命周期已经结束了。

### 消灭悬垂指针

> 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

Rust完全可以避免，比如我们创建一个悬垂指针的例子:

```text
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
fn main() {
    let ref_str = dangle();
}
```

会产生错误:

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:6:16
  |
6 | fn dangle() -> &String {
  |                ^ help: consider giving it a 'static lifetime: `&'static`
```

这是因为在dangle执行完后，s会被释放，我们试图引用一个被释放的值，但是这在rust中是不允许的，它会检测生命周期\(lifetime\)，这个再后面我们会介绍。

可以这么修改:

```text
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
fn main() {
    let ref_str = no_dangle();
    println!("{}", ref_str);
}
```

因为s的所有权被转移给了ref\_str，所以值没有被释放，只是释放了s这个指针而已。

## 引用总结

引用的使用方式:

* 引用必须，也一定总是有效的
* 任意时刻，至多只有一个可变引用，或者多个不可变引用

## 字符串slice

字符串 slice（string slice）是 String 中一部分值的引用，比如:

```text
fn main() {
    let mystr = String::from("hello rust");
    let hello = &mystr[0..5];
}
```

注意hello的类型时&str,必须加上&，这是因为在编译期间是不知道str的具体大小的，只能返回一个指针.slice不是对整个 String 的引用，而是对部分 String 的引用。

在内存中，看起来像这样:

![Slice](../.gitbook/assets/4-2-slice.svg)

### 字符串的字面值就是slice

```text
let s = "Hello, world!";
```

这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。

### 实现一个取字符串第一个单词的例子

遇到空格表示是第一个单词：

```text
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mystring = String::from("hello rust");
    let word = first_word(&mystring[..]);
    println!("{}", word);
    let mystr = "hello rust";
    let word_str = first_word(&mystr);
    println!("{}", word_str);
}
```

可以看出first\_word函数，可以适用于String的slice作为参数，以及字符串字面量，这是因为他们传入的参数都是&str类型。

## 总结

所有权系统决定了Rust的工作方式如此与众不同，确保了在编译时就可获取内存的安全。在后面我们会继续介绍这些概念，这将贯穿本书。

![](../.gitbook/assets/JOJO-TO-BE-CONTINUED.gif)

