/// 덧셈을 수행하는 함수
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 두 수를 곱하는 함수
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 같은 모듈 내의 다른 함수에서 상대 경로 사용
pub fn add_and_multiply(a: i32, b: i32) -> i32 {
    // crate: 루트 모듈을 기준으로 경로를 시작
    // super: 현재 모듈의 상위 모듈을 기준으로 경로를 시작
    // self: 현재 모듈을 기준으로 경로를 시작
    let sum = self::add(a, b);
    multiply(sum, 2)
}
