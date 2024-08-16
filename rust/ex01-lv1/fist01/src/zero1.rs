pub fn declare_variable() {
    // 변수 선언 (기본적으로 불변)
    let x = 5;
    println!("x: {}", x);

    // x는 불변이므로 값을 변경할 수 없음
    // x = 10; // 컴파일 에러

    // 가변 변수 선언
    let mut y = 10;
    println!("초기 y: {}", y);

    // 가변 변수는 값 변경 가능
    y = 15;
    println!("변경된 y: {}", y);

    // 타입을 명시적으로 지정한 변수 선언
    let z: i32 = 20;
    println!("z: {}", z);

    // 타입 추론
    let a = 2.5; // 컴파일러가 자동으로 f64로 추론
    println!("a: {}", a);

    // 변수의 Shadowing (덮어쓰기)
    // 변수 shadowing은 새로운 변수를 생성하는 것이지 기존 변수의 값을 변경하는게 아님
    // 이 과정에서 기존 변수는 숨겨지고(shadowed) 새로운 변수만 보이게 됨
    let x = "Hello";
    // let x: &str = "Hello"; // 이렇게 쓸수도 있음.
    // &str은 고정된 길이의 문자열 조각, 보통 문자열 리터럴을 저장할때 사용
    // let x: String = String::from("Hello"); // 이렇게 쓰기도 가능
    // String은 힙에 저장되는 가변적인 길이의 문자열 타입. String은 메모리 할당이
    // 가능하므로 크기를 동적으로 조정할 수 있음
    println!("덮어쓰기 된 x: {}", x);

    // 상수 선언 (컴파일 타임에 결정됨, 불변)
    // 상수를 선언할땐 자료형을 명시적으로 붙여야 함
    const MAX_POINTS: u32 = 100_000;
    println!("상수 MAX_POINTS: {}", MAX_POINTS);
}
