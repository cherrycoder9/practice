/*
    # 함수 포인터와 트레이트
    - 함수 포인터는 함수의 메모리 주소를 가리키는 포인터
    - 함수를 변수처럼 다룰 수 있게 함
    - 트레이트는 공통된 동작을 정의하는 인터페이스
    - 여러 타입에서 트레이트를 구현해 다형성 제공
*/

pub fn main() {
    // 함수 포인터를 변수에 할당
    let add_fn: fn(i32, i32) -> i32 = add;
    let result = apply_operation(5, 3, add_fn);
    println!("덧셈 결과: {}", result); // 덧셈 결과 출력

    // 트레이트를 구현한 구조체 사용
    let multiplier = Multiplier { factor: 3 };
    let multiplied = apply_trait_operation(4, &multiplier);
    println!("곱셈 결과: {}", multiplied); // 곱셈 결과 출력
}

// 단순 덧셈 함수 정의
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 고차 함수, 두 정수와 함수 포인터를 받아 연산을 적용
fn apply_operation(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(a, b) // 함수 포인터를 호출해 결과 반환
}

// 트레이트 정의, 연산을 수행하는 메서드 포함
trait Operation {
    fn apply(&self, a: i32) -> i32;
}

// 트레이트를 구현한 구조체 정의
struct Multiplier {
    factor: i32, // 곱셈 인자 저장
}

impl Operation for Multiplier {
    fn apply(&self, a: i32) -> i32 {
        a * self.factor // 곱셈 연산 수행
    }
}

// 트레이트를 사용하는 함수 정의
// &dyn: 러스트에서 트레이트 객체를 참조할 때 사용하는 구문
// 트레이트 객체는 동적 디스패치(dynamic dispatch)를 통해
// 트레이트를 구현한 다양한 타입의 값을 다룰 수 있게 해줌
fn apply_trait_operation(a: i32, op: &dyn Operation) -> i32 {
    op.apply(a) // 트레이트 메서드 호출
}
