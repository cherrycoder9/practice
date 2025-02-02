struct TextPair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> TextPair<'a, 'b> {
    fn new(a: &'a str, b: &'b str) -> Self {
        TextPair {
            first: a,
            second: b,
        }
    }

    fn longest(&self) -> &str {
        if self.first.len() > self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let string1 = String::from("Rust");
    let result;
    let string2 = String::from("C++");
    let pair = TextPair::new(&string1, &string2);
    result = pair.longest();

    println!("Longest: {}", result);
}
