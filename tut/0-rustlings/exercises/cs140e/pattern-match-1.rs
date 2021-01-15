// FIXME: Make me compile! Diff budget: 2 lines.

// Do not change this definition.
macro_rules! is_enum_variant {
    ($v:expr, $p:pat) => {
        if let $p = $v {
            true
        } else {
            false
        }
    };
}

enum MyEnum {
    A(String),
    B(String),
}

fn matcher(val: &MyEnum) -> &str {
    match &val {
        MyEnum::A(string) => string.as_str(),
        MyEnum::B(string) => string.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_matcher() {
        let test = MyEnum::A(String::from("abc"));
        assert!(is_enum_variant!(test, MyEnum::A { .. }));
        assert_eq!(String::from("abc"), matcher(&test));
    }

    #[test]
    fn b_matcher() {
        let test = MyEnum::B(String::from("abc"));
        assert!(is_enum_variant!(test, MyEnum::B { .. }));
        assert_eq!(String::from("abc"), matcher(&test));
    }
}
