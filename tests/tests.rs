#![allow(unused)]

use macro_gpt::*;

struct Something;

// generated by: gpt_inject!("Make a trait defining a method called `foo` that prints hello world to the console and have `Something` implement it")

trait HelloWorld {
    fn foo(&self);
}

impl HelloWorld for Something {
    fn foo(&self) {
        println!("Hello, world!");
    }
}

// end of generated code
