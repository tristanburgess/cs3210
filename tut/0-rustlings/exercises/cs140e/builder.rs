// FIXME: Make me pass! Diff budget: 30 lines.

use std::fmt;

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<T: Into<String>>(&mut self, ins: T) -> &mut Self {
        self.string = Some(ins.into());
        self
    }

    fn number(&mut self, inu: usize) -> &mut Self {
        self.number = Some(inu);
        self
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        if let Some(string) = &self.string {
            out.push_str(&format!("{}", string));
            if let Some(number) = self.number {
                out.push_str(&format!(" {:?}", number));
            }
        } else if let Some(number) = self.number {
            out.push_str(&format!("{:?}", number));
        }

        write!(f, "{}", out)
    }
}

// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default().string("heap!".to_owned()).to_string();

    assert_eq!(c, "heap!");
}
