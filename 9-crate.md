# 9 Crate

Crate 是 Rust 的编译单元，可以这么理解：
只要调用了 rustc some_file.rs 这个命令， some_file 就会被认为是**crate**文件。 如果 some_file.rs 中包含 mod，那么 mod 中的语句会在编译之前被插入到 mod 声明的地方。

> 简单来说，模块 mod 不会被单独编译，只有 crates 才会。

一个 crate 可以被编译为一个二进制文件(binary)或者库(library),默认情况 rustc 会产生一个二进制形式。通过传入*--crate-type lib*,可以修改为编译成库。
