use std::array;

pub fn main() {
    // 스칼라 타입은 단일 값을 나타내는 기본 데이터 타입
    // 러스트에서 가장 기본적인 데이터를 표현하는 방식임
    // 주로 숫자, 문자, 불리언과 같은 단일 값을 다루는데 사용
    // 각 타입은 메모리에서 고정된 크기를 가짐
    scalar_types();

    // 복합 타입은 여러 개의 값을 하나로 묶어 관리할 수 있는 데이터 타입
    // 구조체(struct), 열거형(enum), 배열(array), 튜플(tuple)
    compound_types();

    // 튜플은 서로 다른 타입의 여러 값을 하나로 묶어 저장할 수 있는 복합 타입
    // 고정된 크기를 가지며 각 요소는 위치를 통해 접근할 수 있음
    // 함수에서 여러 값을 반환하거나, 관련된 데이터를 그룹화할때 사용
    // 선언시 타입을 명시해야 함
    tuples();

    // 배열은 동일한 타입의 여러 값을 연속적으로 저장하는 복합 타입, 고정 크기
    // 배열의 크기는 컴파일 타임에 결정되며, 한 번 선언된 배열 크기는 변경 불가
    // 배열은 인덱스를 통해 요소에 적븐함, 반복문과 함께 사용해 데이터 처리 가능
    // 효율적인 메모리 사용과 빠른 접근 속도로 성능이 중요한 상황에서 사용
    arrays();
}

fn scalar_types() {
    // 불리언 타입: 참 또는 거짓 값을 가짐
    let is_active: bool = true;
    println!("상태: {}", is_active);

    // 정수 타입: 부호 있는 정수(i32)와 부호 없는 정수(u32) 이외 다양한 크기 있음
    let age: u32 = 30;
    println!("나이: {}", age);

    // 부동 소수점 타입: 소수점을 포함한 숫자(f64)
    let temperature: f64 = 36.6;
    println!("온도: {}", temperature);

    // 문자 타입: 유니코드 문자 하나를 저장
    let grade: char = 'A';
    println!("등급: {}", grade);
}

struct Person {
    name: String,
    age: u32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn compound_types() {
    // 구조체 예제
    let person = Person {
        name: String::from("홍길동"),
        age: 25,
    };
    println!("이름: {}, 나이:{}", person.name, person.age);

    // 열거형 예제
    // let msg = Message::Quit;
    // let msg = Message::Move { x: 10, y: 20 };
    let msg = Message::Write(String::from("안녕"));
    // let msg = Message::ChangeColor(100, 200, 200);
    match msg {
        Message::Quit => println!("프로그램 종료"),
        Message::Move { x, y } => println!("이동 위치: ({}, {})", x, y),
        Message::Write(text) => println!("메시지 작성: {}", text),
        Message::ChangeColor(r, g, b) => println!("색상 변경: RGB({}, {}, {})", r, g, b),
    }
}

fn tuples() {
    // 튜플 선언, 서로 다른 타입의 값들을 묶음
    let person: (&str, u32, char) = ("이순신", 40, 'B');

    // 튜플의 각 요소에 접근하려면 .과 인덱스 사용
    println!("이름: {}", person.0);
    println!("나이: {}", person.1);
    println!("등급: {}", person.2);

    // 함수에서 튜플을 반환
    let coordinates = get_coordinates();
    println!("좌표: ({}, {})", coordinates.0, coordinates.1);
}

fn get_coordinates() -> (i32, i32) {
    (5, 10)
}

fn arrays() {
    // 배열 선언, 동일한 타입의 5개 정수 저장
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // 배열의 각 요소에 접근, 인덱스를 사용해 출력
    println!("첫 번째 숫자: {}", numbers[0]);
    println!("마지막 숫자: {}", numbers[4]);

    // 배열 길이 출력
    println!("배열 길이: {}", numbers.len()); // 5

    // 배열을 반복하면서 모든 요소 출력
    // 다른 언어처럼 in numbers 형태로 사용할 수 없음. 러스트에서는
    // 모든 값이 소유자(owner)를 가지며, 변수를 통해 그 소유권이
    // 이동하거나 빌릴 수 있기 때문임, 배열을 직접 순회하려고 하면
    // 소유권 문제가 발생하기 때문에 빌림 개념을 사용해 numbers 요소들을 참조함
    // 그렇다고 number in &numbers로 쓰게되면, number는 값 자체가 아닌
    // 참조자(reference) 이므로, 루프 내에서 값을 변경할 수 없고 인덱스에 접근하기도 어려움
    // 따라서, 이터레이터를 사용해 컬렉션을 효율적으로 순회할 수 있음
    // numbers.iter()는 numbers에 대한 이터레이터를 반환하며,
    // .enumerate()를 통해 각 요소의 인덱스와 값을 함께 얻을 수 있음
    for (index, number) in numbers.iter().enumerate() {
        println!("인덱스 {}: {}", index, number);
    }

    // 고정된 크기의 배열과 슬라이스 사용
    let slice: &[i32] = &numbers[1..4];
    println!("슬라이스: {:?}", slice); // 슬라이스: [2, 3, 4]
}
