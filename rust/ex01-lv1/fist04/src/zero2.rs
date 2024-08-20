impl Car {
    // 연관 함수 정의 (생성자 역할)
    // 생성자를 impl 안에 구현하는 이유는 러스트의 언어 설계 철학과 관련 있음
    // 러스트는 객체 지향 프로그래밍을 지원하지만, 엄격한 의미에서 클래스 개념은 없음
    // Rust에서 구조체는 단순히 필드의 모음이고 메서드나 함수와 같이
    // 동작을 추가하려면 impl 블록을 사용해야 함, 생성자도 impl 에서 정의함
    // 하지만 생성자라고 부르지는 않으며, new 같은 이름의 함수가 암묵적 생성자임
    pub fn new(year: u32, company: String, name: String) -> Car {
        // 구조체 인스턴스를 Car 형태로 반환
        Car {
            year,
            company,
            name,
        }
    }
}
pub struct Car {
    pub year: u32,
    pub company: String,
    pub name: String,
}
