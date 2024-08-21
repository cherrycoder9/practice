// 열거형과 Option, Result
// 일반적인 에러 처리와 선택적인 값 처리를 위해 사용
// Option 열거형
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

pub fn ex_enum4() {
    let result = divide(10.0, 2.0);

    match result {
        Some(value) => println!("결과: {}", value),
        None => println!("0으로 나눌 수 없음"),
    }
}
