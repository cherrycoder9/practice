use std::{
    fs::File,
    io::{self, Read},
};

/// 파일에서 내용을 읽는 함수 정의
fn read_username_from_file() -> Result<String, io::Error> {
    // 파일을 열고 에러가 발생하면 호출자에게 전파
    let mut file = File::open("username.txt")?; // 에러 발생시 전파
    println!("에러 발생후 출력");
    let mut username = String::new();
    file.read_to_string(&mut username)?;

    // 성공적으로 읽은 사용자 이름 반환
    Ok(username)
}

pub fn main() {
    match read_username_from_file() {
        Ok(name) => println!("사용자 이름: {}", name),
        Err(e) => println!("에러 발생: {}", e),
    }
}
