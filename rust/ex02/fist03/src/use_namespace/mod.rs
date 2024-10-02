mod use_namespace {
    pub mod math_operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }

    pub mod string_operations {
        pub fn concatenate(a: &str, b: &str) -> String {
            format!("{}{}", a, b) // 두 문자열을 연결해 반환
        }
    }
}

// math_operations 모듈의 함수 가져오기
use use_namespace::math_operations::{add, multiply};
// string_operations::concatenate 함수에 별칭 지정
use use_namespace::string_operations::concatenate as concat;

pub fn main() {
    let sum = add(5, 3);
    let product = multiply(4, 6);
    let combined = concat("헬로", "러스트");

    println!("합: {}", sum);
    println!("곱: {}", product);
    println!("연결된 문자열: {}", combined);
}
