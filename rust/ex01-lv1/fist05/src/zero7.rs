// Enum with Associated Data and Dynamic Behavior
// 여러 가지 형태의 변환기를 나타내는 enum 구현하기
// 각 변환기에 맞는 데이터를 처리하도록 trasform 함수 구현

enum Transformer {
    UpperCase(String),
    Square(i32),
    Inverted(bool),
}

fn transform(transformer: Transformer) -> String {
    match transformer {
        Transformer::UpperCase(s) => s.to_uppercase(),
        Transformer::Square(n) => (n * n).to_string(),
        Transformer::Inverted(b) => (!b).to_string(),
    }
}

pub fn main() {
    let t1 = Transformer::UpperCase(String::from("rust"));
    let t2 = Transformer::Square(4);
    let t3 = Transformer::Inverted(true);

    println!("{}", transform(t1));
    println!("{}", transform(t2));
    println!("{}", transform(t3));
}
