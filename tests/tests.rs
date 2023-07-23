use macro_gpt::*;

struct MyStruct {
    x: i32,
}

gpt!(
    "implement PartialEq for an already existing struct called MyStruct that \
    has a single i32 field called x (do not include the struct in your output)."
);
