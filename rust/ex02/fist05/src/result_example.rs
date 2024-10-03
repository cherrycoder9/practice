/// Result 열거형의 구조를 보여주는 예제 함수
fn read_number_from_string(s: &str) -> Result<i32, String> {
    // 문자열을 정수로 변환 시도
    match s.parse::<i32>() {
        Ok(num) => Ok(num), // 변환에 성공하면 Ok 반환
        Err(_) => Err(String::from("유효하지 않은 숫자 형식입니다.")), // 실패시 Err 반환
    }
}

pub fn main() {
    let valid_number = "42";
    let invalid_number = "abc";

    // 유효한 숫자 변환 시도
    match read_number_from_string(valid_number) {
        Ok(num) => println!("변환된 숫자: {}", num), // 성공시 출력
        Err(e) => println!("에러: {}", e),           // 실패시 에러 메시지 출력
    }

    // 유효하지 않은 숫자 변환 시도
    match read_number_from_string(invalid_number) {
        Ok(num) => println!("변환된 숫자: {}", num),
        Err(e) => println!("에러: {}", e),
    }
}
