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
