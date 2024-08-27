// Traits: 타입이 특정한 동작을 구현하도록 하는 방법을 정의함
// 다른 언어에서 인터페이스 또는 추상 클래스와 유사한 개념
// Traits는 공통 기능을 추상화해서 여러 타입에서 일관되게 사용할 수 있게 함

// Trait 정의
trait Summable {
    fn sum(&self) -> i32;
}

// Trait 구현
struct Point {
    x: i32,
    y: i32,
}

impl Summable for Point {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

pub fn main() {
    let point = Point { x: 5, y: 10 };
    println!("좌표 합: {}", point.sum());
}
