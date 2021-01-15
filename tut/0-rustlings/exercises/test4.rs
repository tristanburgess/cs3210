// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

#[macro_export]
macro_rules! my_macro {
    ($val:expr) => {{
        let mut out = String::from("Hello ");
        out.push_str(&$val.to_string());
        out
    }};
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }

    println!("{}", my_macro!(-125));
    println!("{}", my_macro!('v'));
}
