// Result 열거형
// 작업이 성공할 때 Ok, 실패할 때 Err 값을 반환
// Result는 파일 I/O, 네트워킹 등의 작업에서 에러 처리시 자주 사용
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

pub fn ex_enum5() {
    match read_file("hello.txt") {
        Ok(contents) => println!("파일 내용: {}", contents),
        Err(e) => println!("파일 읽는 중 오류 발생: {}", e),
    }
}
