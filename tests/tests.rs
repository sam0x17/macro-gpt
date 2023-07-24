#![allow(unused)]

use macro_gpt::*;

// struct MyStruct {
//     x: i32,
// }

// gpt!(
//     "implement PartialEq for an already existing struct called MyStruct that \
//     has a single i32 field called x."
// );

struct Something;

// gpt_inject!("something else");

// generated by: gpt_inject!("Make a trait defining a method called `foo` that prints hello world to the console and have `Something` implement it")
trait Hello {
    fn foo(&self) {
        println!("Hello, world!");
    }
}

impl Hello for Something {}
// end of generated code
