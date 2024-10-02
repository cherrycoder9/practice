/*
    # 재귀를 이용한 피보나치 수열 계산

    - 함수는 제네릭을 사용하지 않고 단순한 정수 타입만 처리
    - 반환 타입은 u64로 설정할 것
    - 정수 n을 매개변수로 받고, 해당 피보나치 수를 반환
*/

use std::io;

/// 피보나치 수를 재귀적으로 계산하는 함수
fn calculate_fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0; // n이 0일 경우 0 반환
    } else if n == 1 {
        return 1; // n이 1일 경우 1 반환
    }
    // 재귀 호출을 통해 피보나치 수 계산
    calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}

pub fn main() {
    println!("피보나치 수열 계산기"); // 프로그램 시작 메시지 출력
    println!("몇 번째 피보나치 수를 계산하겠습니까?");

    let mut input = String::new(); // 사용자 입력을 저장할 가변 문자열 변수 생성

    // 사용자로부터 입력을 읽어들임
    io::stdin()
        .read_line(&mut input) // 표준 입력으로부터 한 줄 읽기
        .expect("입력 읽기 실패"); // 입력 실패시 에러 메시지 출력

    // 입력된 문자열을 숫자로 파싱
    let trimmed = input.trim(); // 입력 문자열의 공백 제거
    let number: u32 = match trimmed.parse() {
        // 문자열을 u32 타입으로 변환 시도
        Ok(num) => num, // 변환 성공시 num 변수에 저장
        Err(_) => {
            // eprintln!: 터미널이나 콘솔의 오류 출력 스트림(stderr)으로 메시지를 보냄
            // 왜 println!과 구분하는 걸까?
            // 표준 출력과 오류 출력을 분리함으로써 로그를 효율적으로 관리할 수 있음
            // 오류 메시지만 별도로 파일에 기록하거나 모니터링 시스템으로 전송하는 등
            // 쉘 스크립트나 다른 프로그램과 파이프라인을 구성할 때,
            // 표준 출력과 오류 출력을 분리해 처리할 수 있음
            eprintln!("유효한 정수를 입력하세요."); // 변환실패시 에러 메시지 출력
            return; // 프로그램 종료
        }
    }; // 숫자 변환 처리 종료

    // 입력된 숫자가 너무 큰 경우 재귀 깊이 초과를 방지
    if number > 40 {
        eprintln!("입력값이 너무 큽니다. 40 이하의 숫자를 입력하세요.");
        return;
    }

    let result = calculate_fibonacci(number);
    println!("피보나치 수열의 {}번째 수는 {}입니다.", number, result); // 결과 출력
}
