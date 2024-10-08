pub fn main() {
    // 클로저는 주변 환경의 변수를 캡처할 수 있는 함수 형태의 표현식
    // 다른 언어의 람다와 유사하지만, 러스트에선 소유권과 관련된 특별한 동작을 가짐

    // 기본적인 클로저 정의
    let add_one = |x: i32| -> i32 {
        // x는 매개변수, -> i32는 반환 타입임
        x + 1
    };

    // 클로저 호출
    let result = add_one(5);
    println!("결과: {}", result); // 6 출력

    // 클로저는 주변 환경의 변수를 캡처할 수 있음
    let base_value = 10;
    let add_base = |x: i32| -> i32 {
        // 이 클로저는 base_value를 캡처해 사용함
        x + base_value
    };

    let result = add_base(5);
    println!("결과: {}", result); // 15 출력

    // 캡처 방법에 따른 차이점
    // 러스트에서 클로저는 변수의 소유권을 이동하거나, 불변/가변 참조로 캡쳐할 수 있음
    // 이 동작 방식은 클로저가 사용하는 환경에 따라 자동으로 결정됨
    // 소유권 이동 클로저 예제
    let string_value = String::from("안녕하세요");
    let print_string = move || {
        // move 키워드를 사용해 클로저가 string_value의 소유권을 가져옴
        // 이후 string_value는 더 이상 사용될 수 없음
        println!("캡처된 문자열: {}", string_value);
    };

    // 클로저 호출
    print_string();

    // Fn, FnMut, FnOnce 트레이트 설명
    // 클로저는 세 가지 트레이트(Fn, FnMut, FnOnce)를 구현할 수 있음
    // - Fn: 클로저가 환경을 불변으로 캡처할 때 사용됨
    // - FnMut: 클로저가 환경을 가변으로 캡처할 때 사용됨
    // - FnOnce: 클로저가 환경의 소유권을 이동할 때 사용됨

    // FnMut 클로저 예제
    let mut value = 0;
    let mut add_to_value = |x: i32| {
        // 클로저가 value를 가변으로 캡처하므로 FnMut 트레이트를 구현함
        value += x;
        println!("현재 값: {}", value);
    };

    add_to_value(5); // 5
    add_to_value(3); // 8
}
