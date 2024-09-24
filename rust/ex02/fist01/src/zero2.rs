use std::io; // 표준 입출력 라이브러리 사용

pub fn main() {
    let first_number = read_integer("정수1 입력: ");
    let mut mutable_number = first_number;
    let increment_value = read_integer("정수2 입력: ");
    mutable_number += increment_value;
    println!("최종 값: {}", mutable_number);
}

// 사용자로부터 입력을 받고 정수로 변환하는 함수
fn read_integer(prompt: &str) -> i32 {
    loop {
        // 콘솔에 입력 메시지 출력
        print!("{}", prompt);

        // 버퍼를 플러시해 메시지가 즉시 표시되도록 함
        // io::stdout() 함수는 표준 출력 스트림을 반환함
        // 주로 콘솔에 데이터를 출력할 때 사용됨
        // 프로그램이 실행되는 터미널이나 콘솔 창으로 데이터를 보냄
        // .expect() 함수는 Result 타입을 처리할 때 사용되는 메서드
        // Result가 Ok인 경우 내부 값 반환, Err인 경우 패닉 상태로 만들어 오류 메시지 출력
        io::Write::flush(&mut io::stdout()).expect("출력 버퍼를 플러시할 수 없음");

        // 사용자 입력을 저장할 가변 문자열 변수 선언
        let mut input = String::new();

        // 사용자로부터 한 줄의 입력을 읽음
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // 입력된 문자열의 앞뒤 공백 제거 후 파싱 시도
                match input.trim().parse::<i32>() {
                    Ok(num) => return num,                           // 파싱 성공시 정수 반환
                    Err(_) => println!("유효한 정수를 입력하세요."), // 파싱 실패시 오류 메시지
                }
            }
            Err(_) => println!("입력을 읽는 중 오류 발생"), // 입력 읽기 실패시 오류 메시지
        }
    }
}
