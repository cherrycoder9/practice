use zero1::Person;

mod zero1;
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
}
