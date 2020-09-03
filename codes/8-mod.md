# 8 模块\(Module\)

## 8 模块\(Module\)

## 模块

模块类似于 Java 或者 Go 的 package，它可以把不同文件的代码按照**逻辑**组织在一起，并且管理可见性\(public/private\)。

模块包含：函数，结构体，traits，impl 甚至包括其他模块。

### 模块定义和使用

模块以 mod 开头，它声明了一个 _模块名字::_ 开头的命名空间. 从下面的例子来看，其相对关系:

```text
// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the mode specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}
```

### 模块中结构体可见性

模块中结构体的字段和 impl 中的函数会声明单独的可见性。

```text
mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
```

### 使用 use 来简化引用模块全路径

```text
mod deeply {
    pub mod nested {
        pub fn func() {
            println!("called deeply nested func",);
        }
    }
}

use crate::deeply::nested::func;
use crate::deeply::nested::func as other_func;

fn main() {
    func();
    other_func()
}
```

可以看出,还可以使用 as 重命名

### supper 和 self

使用 super 直接引用上一级 mod 模块，使用 self 来引用本级 mod 中的函数或模块。 举例来看:

```text
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();

        // We can also use `self` to access another module inside `my`:
        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();

        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

### 文件层级

#### 第一种方式，直接在 src 目录下创建同名的包

文件代码见:[hirechy](https://github.com/dajuguan/rust_blockchain/tree/master/codes/src/hirechy) 我们创建一个文件

```text
cargo new hirechy
cd hirechy/src
touch my.rs
```

建立如下的目录:

```text
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── my.rs
```

文件名:my.rs

```text
pub fn function() {
    println!("called my::function()");
}
```

文件名:main.rs

```text
mod my;
fn main() {
    my::function();
}
```

#### 第二种方式，创建与包名相同的文件夹，并在该文件夹下创建 mod.rs 文件

同样我们创建相应的目录\(文件代码见:[hirechy2](https://github.com/dajuguan/rust_blockchain/tree/master/codes/src/hirechy2) \):

```text
hirechy2
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── my
│       ├── mod.rs
│       └── nested.rs
```

文件 src/main.rs

```text
mod my;
fn main() {
    my::function();
    my::nested::function();
}
```

文件 my/mod.rs

```text
pub mod nested;

pub fn function() {
    println!("call my::function()",);
}
```

文件 my/nested.rs

```text
pub fn function() {
    println!("called my::nest::function()",);
}
```

