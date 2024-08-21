// Option 열거형
// 값이 있을수도 있고 없을수도 있는 경우를 표현함
// Some은 값이 있을때, None은 값이 없을때 사용
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
