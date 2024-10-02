mod re_export {
    pub mod internal {
        pub fn internal_function() {
            println!("내부 함수 호출");
        }

        pub struct InternalStruct {
            pub value: i32, // 퍼블릭 필드
        }
    }

    // internal 모듈의 항목을 외부로 재수출
    pub use internal::internal_function; // 함수 재수출
    pub use internal::InternalStruct; // 구조체 재수출
}

pub fn main() {
    // 재수출된 함수 호출
    re_export::internal_function();

    // 재수출된 구조체 사용
    let instance = re_export::InternalStruct { value: 42 };
    println!("구조체 값: {}", instance.value);
}
