// 열거형과 메서드
// 열거형에 메서드 정의할 수 있음
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

pub fn ex_enum3() {
    let coin = Coin::Dime;
    println!("코인의 가치는 {} 센트입니다.", coin.value_in_cents());
}
