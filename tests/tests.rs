use macro_gpt::*;

struct MyStruct {
    x: i32,
}

gpt!("implement PartialEq for a struct called MyStruct that has a single i32 field called x");
