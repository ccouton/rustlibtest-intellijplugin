#![recursion_limit = "128"]
#[macro_use(quick_error)]
extern crate quick_error;

pub mod mod1;
pub mod errors;

pub fn test2() {
    mod1::test();
}