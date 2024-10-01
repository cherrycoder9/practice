/*
    # 재귀 함수 및 반복 처리
    - 재귀 함수는 자기 자신을 호출해 문제를 해결하는 함수
    - 복잡한 문제를 단순한 하위 문제로 분할할 때 유용함
*/

pub fn main() {
    // 재귀 함수를 사용해 팩토리얼 계산
    let num = 5;
    let fact = factorial_recursive(num);
    println!("{}의 팩토리얼(재귀): {}", num, fact); // 재귀 결과 출력

    // 반복을 사용해 팩토리얼 계산
    let fact_iter = factorial_iterative(num);
    println!("{}의 팩토리얼(반복): {}", num, fact_iter); // 반복 결과 출력
}

// 재귀 함수를 사용해 팩토리얼 계산
fn factorial_recursive(n: u32) -> u32 {
    if n == 0 {
        1 // 0!는 1
    } else {
        n * factorial_recursive(n - 1) // 재귀 호출
    }
}

// 반복을 사용해 팩토리얼 계산
fn factorial_iterative(n: u32) -> u32 {
    let mut result = 1;
    // .. : 시작 값은 포함하지만 끝 값은 포함하지 않음
    // ..= : 시작 값과 끝 값 모두 포함
    for i in 1..=n {
        result *= i; // 각 단계에서 결과를 곱함
    }
    result // 최종 결과 반환
}
