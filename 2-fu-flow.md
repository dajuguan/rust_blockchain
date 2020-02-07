# 2 函数与控制流

# 函数

在Rust中，main函数式程序的入口；函数的定义以生命关键字fn开始,如下形式:

```
fn main() {
    func(2, 1.2)
}

fn func(u: u32, f: f64) {
    println!("First Func!");
}
```

- 函数可以再任意位置定义，只要定义就可以，没必要区分前后；
- 参数需要制定类型，有多个参数用逗号隔开
  
# 具有返回值的函数
使用->来制定返回值的类型，不要要制定返回值的名字；有多个返回值可以使用tuple。如下例：

```
fn main() {
    let x = fun(2);
    println!("{}", x);
    let (x, y) = func_with_mutiple_returns(2, 1.2);
    println!("{} {}", x, y);
}
fn fun(x: u32) -> u32 {
    return x;
}
fn func_with_mutiple_returns(u: u32, f: f64) -> (u32, f64) {
    (u, f)
}
```
> Rust是基于表达式的语言，表达式可以直接返回，而不需要return语句。

# 控制流

Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。

# if表达式

if允许根据条件执行不同的代码分支，并可以返回表达式的值.需要注意的是if判断条件，不像ruby或者python等，它不会直接转换为bool，必须传bool值才可以。

```
    let cond = 5;
    if true {
        println!("...");
    } else if cond > 5 {
        println!("else");
    }
```
同时，if还可以作为表达式返回，因此可以和let一起使用:
```
    let x = if true { 5 } else { 6 };
```

# 循环
Rust 有三种循环：loop、while 和 for。

## loop循环
loop后面接一个块，可以一直执行，直到遇到break语句:

```
    let mut counter = 0;
    loop {
        counter += 1;
        println!("{}", counter);
        if counter == 10 {
            break;
        }
    }
```

由于rust是基于表达式的，因此我们还可以使用loop返回表达式:
```
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", res);
```

同时，rust也支持loop的嵌套，并且给定标志符:
```
    'a: loop {
        println!("loop a..");
        'b: loop {
            println!("loop b..");
            'c: loop {
                println!("loop c..");
                break 'b;
            }
        }
    }
```
这段代码会不断的输出:
```
loop a..
loop b..
loop c..
loop a..
loop b..
loop c..
```


## while循环
直接将判断放在while后面，避免使用if else语句
```
    while counter < 5 {
        println!("{}", counter);
        counter += 1;
    }
```

## for循环
为了防止在循环中出现数组越界等行为，rust支持for循环来访问集合的元素
```
    let a = [10, 11, 12, 13];
    for val in a.iter() {
        println!("{}", val);
    }
```
同时，for也支持使用range来限定执行的次数，通过使用内置的".."来实现：
```
    for number in (1..4) {
        println!("{}!", number);
    }
```
 ![](/imgs/JOJO-TO-BE-CONTINUED.gif)