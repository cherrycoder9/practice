/*
    #[derive(Debug)]
    - 구조체나 열거형에 자동으로 Debug 트레이트를 구현하도록 지시하는 속성
    - Debug 트레이트를 구현하면 println!("{:?}", value)와 같이
    - {:?} 포맷을 사용해 구조체나 열거형의 내용을 출력할 수 있음
    - 에러 발생시 개발자가 에러의 내부 상태나 원인을 쉽게 파악할 수 있게 함
    - std::error:Error 트레이트는 Debug 트레이트를 상속받음
    - 따라서 Error 트레이트를 구현하기 위해선 Debug 트레이트가 반드시 구현되어 있어야 함
    - 에러가 발생했을때, Debug 포맷을 통해 상세한 에러 정보를 로그에 남길수 있음
    - ex) eprintln!("에러 발생: {:?}", e);
*/

use core::fmt;
use std::{fs::File, io::Read, num::ParseIntError};

/// 사용자 정의 에러 타입
#[derive(Debug)]
enum MyError {
    /// 파일 관련 에러
    FileError(String),
    /// 파싱 관련 에러
    ParseError(String),
}

/// `Display` 트레이트 구현
impl fmt::Display for MyError {
    /// 에러 메시지 포맷 정의
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::FileError(msg) => write!(f, "파일 에러: {}", msg),
            MyError::ParseError(msg) => write!(f, "파싱 에러: {}", msg),
        }
    }
}

/// 파일을 읽고 파싱하는 함수 정의
fn read_and_parse_file(filename: &str) -> Result<i32, MyError> {
    // 파일 열기 시도, 파일 열기 에러를 커스텀 에러로 변환
    let mut file = File::open(filename).map_err(|e| MyError::FileError(e.to_string()))?;

    let mut contents = String::new();
    // 파일에서 내용 읽기 시도, 읽기 에러를 커스텀 에러로 변환
    file.read_to_string(&mut contents)
        .map_err(|e| MyError::FileError(e.to_string()))?;

    // 내용 파싱 시도, 파싱 에러를 커스텀 에러로 변환
    let number: i32 = contents
        .trim()
        .parse()
        // 위 parse 함수에 제네릭 타입 파라미터를 명시적으로 지정하거나 (.parse::<i32>())
        // 아래 클로저 매개변수 e에 명시적 타입(ParseIntError)을 지정해야 e에 빨간줄 안뜸
        // 이유: 컴파일러가 map_err 클로저의 매개변수 e에 대해 타입을 추론하지 못했음
        .map_err(|e: ParseIntError| MyError::ParseError(e.to_string()))?;

    Ok(number) // 성공시 파싱된 숫자 반환
}

pub fn main() {
    match read_and_parse_file("number.txt") {
        Ok(num) => println!("파싱된 숫자: {}", num), // 성공시 숫자 출력
        Err(e) => eprintln!("에러 발생: {}", e),     // 에러 메시지 출력
    }
}
