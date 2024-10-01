/*
    # 문자열 처리 함수 구현

    - 사용자로부터 문자열 슬라이스를 입력받음
    - 각 단어의 길이를 계산하고 그 결과를 벡터로 반환
    - 제네릭 타입을 사용하지 않고 단순히 &str 타입을 처리
*/

use std::io;

// 단일 줄 주석
// 코드 내에서 간단한 설명이나 메모를 추가할 때 사용
/// 문서 주석
/// 함수, 구조체, 모듈 등의 API 문서를 작성할 때 사용됨
/// 이 주석은 rustdoc 도구에 의해 HTML 문서로 변환될 수 있음

/// 입력된 문자열의 각 단어 길이를 계산해 벡터로 반환하는 공개 함수
pub fn calculate_word_lengths(input: &str) -> Vec<usize> {
    input // 입력 문자열 참조
        .split_whitespace() // 공백을 기준으로 단어 분리
        .map(|word| word.len()) // 각 단어의 길이를 계산
        .collect() // 결과를 벡터로 수집
}

pub fn main() {
    let mut user_input = String::new(); // 사용자 입력을 저장할 가변 문자열 생성

    println!("문자열을 입력하세요: ");

    // 사용자로부터 한 줄의 입력을 읽음
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            // 입력이 성공적으로 읽혔을 경우
            let trimmed_input = user_input.trim(); // 입력 문자열의 앞뒤 공백 제거

            if trimmed_input.is_empty() {
                // 입력 문자열이 비어있는지 확인
                eprintln!("오류: 입력 문자열이 비어 있습니다.");
                return;
            } // 빈 입력 문자열 확인 종료

            let lengths = calculate_word_lengths(trimmed_input);
            println!("각 단어의 길이: {:?}", lengths); // 계산된 단어 길이 벡터 출력
        }
        // 입력 중 오류가 발생한 경우
        Err(error) => {
            eprintln!("입력 오류: {}", error);
        }
    } // read_line 매치 블록 종료
} // main 함수 종료
