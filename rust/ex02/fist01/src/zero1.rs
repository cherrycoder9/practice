// 변수 선언 기본
pub fn variable_declaration() {
    // 변수 선언은 let 키워드를 사용
    // 변수 선언시 반드시 초기화 해야함
    let x = 5;
    println!("변수 x의 값: {}", x);
}

// 불변 변수
pub fn immutable_variable() {
    // 불변 변수는 기본적으로 값이 변경되지 않음
    // 데이터의 안정성과 예측 가능성 증가
    let y = 10;
    println!("불변 변수 y 초기값: {}", y);

    // 불변 변수는 값을 변경할 수 없음
    // y = 20; // 컴파일 에러 발생
}

// 가변 변수
pub fn mutable_variable() {
    // 가변 변수는 mut 키워드를 사용해 선언
    // 필요에 따라 값의 변경이 가능함
    let mut z = 15;
    println!("가변 변수 z의 초기값: {}", z);

    // 가변 변수의 값 변경
    z = 25;
    println!("가변 변수 z의 변경된 값: {}", z);
}

// 변수 스코프와 생명주기
pub fn variable_scope() {
    // 변수의 스코프는 변수가 유효한 범위를 의미
    // 스코프가 종료되면 변수는 소멸됨
    {
        // 내부 스코프
        let a = 30;
        println!("내부 스코프의 변수 a: {}", a);
    }
    // a는 여기서 소멸됨
    // println!("외부 스코프의 변수 a: {}", a); // 컴파일 에러 발생
}

// 변수 그림자화 (Shadowing)
pub fn variable_shadowing() {
    // 그림자화는 같은 이름의 변수를 재선언해 새로운 값을 할당할 수 있는 기능
    // 변수 재사용에 유용함
    let b = 50;
    println!("처음 선언된 변수 b: {}", b);

    let b = b + 10;
    println!("그림자화된 변수 b: {}", b);

    {
        let b = b * 2;
        println!("내부 스코프의 그림자화된 변수 b: {}", b);
    }

    println!("외부 스코프의 변수 b: {}", b);
}

// 상수(const)와 불변 변수(let)의 차이점
pub fn constants_vs_immutable() {
    // const는 상수로, 컴파일 시점에 값이 결정됨
    // 불변 변수와는 다르게 타입이 명시되어야 함
    const MAX_POINTS: u32 = 100_000;
    let score = 95;
    println!("상수 MAX_POINTS: {}", MAX_POINTS);
    println!("불변 변수 score: {}", score);
}

// 가변성과 안전성 보장
pub fn mutability_safety() {
    // 러스트의 소유권 시스템은 가변성을 안전하게 관리
    // 데이터 경쟁을 방지함
    let mut s = String::from("헬로");
    println!("초기 문자열: {}", s);

    // 가변 참조를 통해 문자열 변경
    s.push_str(", 러스트!");
    println!("변경된 문자열: {}", s);

    // 불변 참조와 가변 참조는 동시에 사용 불가
    let r1 = &s;
    // let r2 = &mut s; // 컴파일 에러
    println!("불변 참조 r1: {}", r1);
}

// 변수의 타입 추론과 명시
pub fn variable_type() {
    // 러스트는 변수의 타입을 대부분의 경우 자동으로 추론
    // 필요시엔 명시적으로 타입 지정 가능함

    // 타입 추론을 통한 변수 선언
    let inferred_integer = 42;
    println!("추론된 정수형 변수: {}", inferred_integer);

    // 명시적 타입 지정
    let explicit_float: f64 = 3.14;
    println!("명시적 실수형 변수: {}", explicit_float);
}

// 상수와 불변 변수의 메모리 배치
pub fn constants_memory_layout() {
    // const는 프로그램 전체에서 고정된 메모리 위치에 저장됨
    // let으로 선언된 불변 변수는 스택에 저장됨
    const GLOBAL_CONST: u32 = 500;
    let local_immutable = 300;
    println!("상수 GLOBAL_CONST: {}", GLOBAL_CONST);
    println!("지역 불변 변수 local_immutable: {}", local_immutable);
}

// 변수와 패턴 매칭
pub fn variable_pattern_matching() {
    // 변수는 패턴 매칭의 일부로 사용되며
    // 복잡한 데이터 구조에서 값을 추출하는데 활용됨
    let (x, y, z) = (1, 2, 3);
    println!("튜플에서 추출된 변수 x: {}, y: {}, z: {}", x, y, z);

    // 러스트의 패턴 매칭은, 모든 패턴에 대해 다뤄야 하기 때문에
    // 아래처럼 작성하면 None의 경우를 다루지 않아 경고 발생
    // let Some(a) = Some(10);
    // println!("옵션 타입에서 추출된 변수 a: {}", a);

    // if let 구문을 사용하거나 match 구문을 사용해야 함
    let maybe_number = Some(10);

    // if let을 사용해 Some(a) 패턴을 매칭
    if let Some(a) = maybe_number {
        println!("옵션 타입에서 추출된 변수 a: {}", a);
    } else {
        println!("옵션 타입이 None입니다.");
    }
}

// 상수
pub fn constants_usage() {
    // 상수는 프로그램 전체에서 변하지 않는 값
    // 주로 고정된 값이나 설정에 사용
    const PI: f64 = 3.14159265;
    let radius = 5.0;
    let circumference = 2.0 * PI * radius;
    println!("반지름 {}의 원주: {}", radius, circumference);
}

// 가변 참조와 빌림
pub fn mutable_reference() {
    // 가변 참조는 데이터의 소유권을 넘기지 않고
    // 값을 변경할 수 있는 방법 제공
    let mut message = String::from("안녕");
    append_world(&mut message);
    println!("변경된 메시지: {}", message);
}

fn append_world(s: &mut String) {
    s.push_str(", 러스트!");
}

// 불변 참조와 가변 참조 규칙
pub fn reference_rules() {
    // 불변 참조와 가변 참조는 동시에 사용 불가능
    // 가변 참조는 단일로만 가능
    let mut data = 100;

    // 불변 참조
    let r1 = &data;
    let r2 = &data;
    println!("불변 참조 r1: {}, r2: {}", r1, r2);

    // 가변 참조는 불변 참조가 끝난 후에만 가능함
    let r3 = &mut data;
    println!("가변 참조 r3: {}", r3);
}

// 변수 재선언과 섀도잉
pub fn variable_redeclaration() {
    // 변수 재선언으로 데이터 변환이나 타입 변경을 유연하게 처리
    let spaces = "       ";
    println!("초기 spaces: '{}'", spaces);

    // 변수 재선언을 통한 타입 변경
    let spaces = spaces.len();
    println!("spaces의 길이: {}", spaces);
}

// 상수의 메타데이터
pub fn constants_naming() {
    // 상수는 항상 대문자와 언더스코어를 사용해 명명함
    const MAX_SPEED: u32 = 250;
    println!("최대 속도: {} km/h", MAX_SPEED);
}

// 스코프 내에서의 가변성 제어
pub fn mutability_scope() {
    // 스코프를 통해 가변성 범위를 제한할 수 있음
    let value = 10;
    println!("초기 값: {}", value);

    {
        let mut value = value;
        value += 5;
        println!("내부 스코프의 값: {}", value);
    }

    println!("외부 스코프의 값: {}", value);
}

// 컴파일 시점의 상수 평가
pub fn const_evaluation() {
    // 상수는 컴파일 시점에 평가되어 최적화됨
    const AREA: u32 = calculate_area(10);
    println!("정사각형의 면적: {}", AREA);
}

const fn calculate_area(side: u32) -> u32 {
    side * side
}

// 변수의 전역 사용과 제한
pub fn global_variable() {
    static GLOBAL_COUNT: u32 = 0;
    println!("전역 변수 GLOBAL_COUNT: {}", GLOBAL_COUNT);

    // 전역 변수는 변경 불가
    // GLOBAL_COUNT = 1; // 컴파일 에러
}

// 변수와 함수 내 가변성
pub fn function_mutability() {
    // 함수 내에서 변수의 가변성을 관리해 데이터 일관성을 유지
    let mut counter = 0;
    increment(&mut counter);
    println!("증가된 counter: {}", counter);
}

fn increment(value: &mut i32) {
    *value += 1;
}
