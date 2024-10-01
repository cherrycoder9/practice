/*
    # 고차 함수와 클로저
    - 고차 함수: 다른 함수를 인수로 받거나 함수를 반환하는 함수
    - 클로저: 환경을 캡쳐할 수 있는 익명 함수, 고차 함수와 함께 자주 사용됨
    - 클로저를 사용하면 함수 내부에서 외부 변수에 접근할 수 있음
*/

// 고차 함수 정의: 벡터와 클로저를 인수로 받음
fn apply_to_vector<F>(vec: &Vec<i32>, func: F) -> Vec<i32>
where
    // where절의 역할: 제네릭 타입에 대한 제약을 명시적으로 표현하는 구문.
    F: Fn(i32) -> i32, // F는 i32를 받아 i32를 반환하는 함수 또는 클로저여야 함
{
    // vec.iter(): 벡터의 각 요소에 대한 반복자를 생성
    // map(|&x| func(x)): 반복자의 각 요소 x에 대해 func 함수를 적용해 변환
    // |&x|: 클로저의 매개변수로 벡터의 각 요소를 참조 형태로 전달받아 값으로 복사
    // func(x): func 함수를 호출해 변환된 값을 생성
    // collect(): 변환된 값을 새로운 벡터로 수집해 반환
    vec.iter().map(|&x| func(x)).collect() // 클로저를 각 요소에 적용
}

pub fn main() {
    // 고차 함수를 정의하고 클로저를 인수로 전달
    let numbers = vec![1, 2, 3, 4, 5];

    // 클로저를 사용해 각 요소에 2를 곱함
    let doubled: Vec<i32> = apply_to_vector(&numbers, |x| x * 2);

    println!("원본 벡터: {:?}", numbers); // 원본 벡터 출력
    println!("2를 곱한 벡터: {:?}", doubled); // 변환된 벡터 출력
}