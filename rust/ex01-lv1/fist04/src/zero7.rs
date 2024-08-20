// 원의 정보와 연산
// 원의 반지름을 저장하는 구조체 정의
// 원 둘레를 계산하는 메서드와 넓이를 계산하는 메서드를 정의
// 구조체의 인스턴스를 생성하고 둘레와 넓이를 출력하는 코드 작성
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
    // 둘레를 계산하는 메서드
    pub fn circumference(&self) -> f64 {
        2.0 * 3.14159 * self.radius
    }
    // 넓이를 계산하는 메서드
    pub fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
