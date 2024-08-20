use zero1::Person;
use zero2::Car;
use zero3::Color;
use zero4::AlwaysEqual;
use zero5::Student;
use zero6::Rectangle;

mod zero1;
mod zero2;
mod zero3;
mod zero4;
mod zero5;
mod zero6;
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

    // 변수가 코드에서 사용되지 않을 것임을 컴파일러에 알리기 위해 변수앞에 _ 붙임
    // 경고를 무시할 수 있음
    let _unit_struct = AlwaysEqual;
    // 별다른 동작은 없지만 타입을 구분할 때 유용함
    println!("유닛 구조체 생성");

    // Student 인스턴스 생성
    let s = Student {
        name: String::from("박하늬"),
        student_id: String::from("409127891"),
        grade: 2,
    };
    println!(
        "이름: {}, 학번: {}, 학년: {}",
        s.name, s.student_id, s.grade
    );

    let r = Rectangle::new(3.5, 3.5);
    println!("넓이: {}", r.area());
}
