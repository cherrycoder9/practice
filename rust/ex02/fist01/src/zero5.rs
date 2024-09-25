pub fn main() {
    // 열거형은 관련된 값들의 집합을 정의할 수 있는 타입
    // 패턴 매칭과 결합해 제어 흐름 가능
    // 각 열거형 변형은 다른 타입의 데이터를 포함할 수 있음
    let current_light = TrafficLight::Green;
    display_light_state(current_light);

    // 구조체는 관련된 데이터를 하나의 복합 타입으로 묶어주는 역할
    // 데이터의 논리적 그룹화
    // 새로운 Person 인스턴스 생성
    let person = Person::new("러스트", 15, "rust@rust.com");
    // 정보 출력
    person.display_info();
}

// 교통 신호등 상태를 나타내는 열거형 정의
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn display_light_state(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("정지하세요"),
        TrafficLight::Yellow => println!("주의하세요"),
        TrafficLight::Green => println!("진행하세요"),
    }
}

// 사람을 나타내는 구조체 정의
struct Person {
    name: String,
    age: u32,
    email: String,
}

// 구조체에 메서드를 구현하는 impl 블록
impl Person {
    // 새로운 Person 인스턴스를 생성하는 연관 함수
    fn new(name: &str, age: u32, email: &str) -> Person {
        Person {
            name: name.to_string(),
            age,
            email: email.to_string(),
        }
    }

    // 사람의 정보를 출력하는 메서드
    fn display_info(&self) {
        println!("이름: {}", self.name);
        println!("나이: {}", self.age);
        println!("이메일: {}", self.email);
    }
}
