/*
    # 클로저와 함수의 차이 비교
    - 클로저는 주변 환경을 캡처할 수 있음
    - 함수는 그러지 못함
*/

// 입력 값을 두 배로 반환하는 함수
fn double_function(x: i32) -> i32 {
    // 함수는 자신의 매개변수 외의 값을 캡처하지 않음
    x * 2 // x의 값을 두 배로 반환
}

pub fn main() {
    // 입력 값을 두 배로 반환하는 클로저
    let factor = 2; // 클로저에서 사용할 외부 변수 factor

    // 클로저는 주변의 변수를 캡처할 수 있음
    let double_closure = |x: i32| -> i32 {
        x * factor // x의 값을 factor만큼 곱해 반환함. factor는 외부에서 캡처된 값
    };

    // 함수 호출
    let result_function = double_function(5);
    println!("함수 결과: {}", result_function);

    // 클로저 호출
    let result_closure = double_closure(5);
    println!("클로저 결과: {}", result_closure);

    // 클로저 추가 예제, 가변 캡처
    let mut count = 0; // 가변 변수 count 정의
    let mut increment = || {
        count += 1; // 클로저는 외부 가변 변수를 캡처하고 수정할 수 있음
        println!("현재 count 값: {}", count);
    };

    increment();
    increment();

    // 클로저는 정의된 환경에 따라 불변참조(&), 가변참조(&mut), 값 캡처 모두 가능
    // 클로저는 유연한 환경 캡처 특성 때문에 람다식으로 사용되며, 함수보다 더 많은 메모리 사용
}
