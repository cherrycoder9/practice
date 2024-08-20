use zero1::Person;
use zero2::Car;
use zero3::Color;

mod zero1;
mod zero2;
mod zero3;
fn main() {
    // 구조체 인스턴스 생성
    let mut p = Person {
        name: String::from("병아리"),
        age: 30,
        email: String::from("병아리@gmail.com"),
    };

    // 출력
    println!("이름: {}, 나이: {}, 이메일: {}", p.name, p.age, p.email);

    // 필드 값 변경
    // 인스턴스 생성시 mut 키워드를 붙여줬어야 값 변경됨
    p.age = 40;
    // 출력
    println!("이름: {}, 나이: {}, 이메일: {}", p.name, p.age, p.email);

    // 메서드 호출
    println!("{}", p.greeting());

    // 연관 함수 호출로 구조체 인스턴스 생성
    let c = Car::new(1986, String::from("KIA"), String::from("Avantte"));
    println!("년도: {}, 회사: {}, 이름: {}", c.year, c.company, c.name);

    let black = Color(0, 0, 0);

    // 필드에 인덱스로 접근
    println!("Red: {}, Green: {}, Blue: {}", black.0, black.1, black.2);
}
