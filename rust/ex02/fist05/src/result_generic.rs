/// 다양한 제네릭 타입을 사용하는 Result 예제 함수
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    // Result나 Option과 같은 열거형의 모든 변형을 처리해야 한다면 match를 사용하는 것이 좋음
    // 단순히 성공 또는 실패 여부만 확인하고 싶을때, 또는 실패시 특정 동작만 수행하고 싶을땐 if
    if denominator == 0.0 {
        // 분모가 0인지 확인
        Err(String::from("분모는 0일 수 없습니다.")) // 분모가 0이면 에러 반환
    } else {
        Ok(numerator / denominator) // 정상적으로 나눗셈 수행
    }
}

pub fn main() {
    // 정상적인 나눗셈 시도
    match divide(10.0, 2.0) {
        Ok(result) => println!("10.0 / 2.0 = {}", result),
        Err(e) => println!("에러: {}", e),
    }

    // 분모가 0인 경우
    match divide(10.0, 0.0) {
        Ok(result) => println!("10.0 / 0.0 = {}", result),
        Err(e) => println!("에러: {}", e),
    }
}
