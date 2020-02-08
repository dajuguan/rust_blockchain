# 10 属性(attibutes)

属性是应用到 module，crate 或者 item 上的原数据(metadata).这个 metadata 可以用来:

- 条件编译
- 设置 crate 名字，版本和类型(binary/library)
- 取消代码提示 lints
- 启用编译器特性(宏，混合导入等)
- 链接到外部 library
- 标记函数为单元测试
- 标记函数为 benchmark 的一部分

Attribute 可以使用不同的语法接受参数:

- #[attribute = "value"]
- #[attribute(key = "value")]
- #[attribute(value)]

上面的语法只能在一个 module 或者 item 上使用，在一个 crate 中全局使用 crate 的语法是:#![crate_attribute]，主要是多了个 _!_。

Attributes 也可以接受多个参数:

```
#[attribute = （value1,value2)]
```

# 不执行的代码(dead_code)

使用*attribute*来禁用编译器默认的 dead_code 代码提示

```
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}
```

# 使用 cfg 条件编译

可以使用两种等价的语句来进行条件编译:

- 在 attribute 位置使用**cfg** attribute:#[cfg(...)]
- 在布尔表达式使用**cfg!** 宏: cfg!(...)

```
// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

```
