mod access_control {
    // 퍼블릭 구조체 정의
    pub struct PublicStruct {
        pub public_field: i32,
        private_field: String,
    }

    impl PublicStruct {
        // 구조체 인스턴스 생성
        pub fn new(public: i32, private: &str) -> PublicStruct {
            PublicStruct {
                public_field: public,               // 퍼블릭 필드 초기화
                private_field: private.to_string(), // 프라이빗 필드 초기화
            }
        }

        // 프리이빗 필드 접근
        pub fn get_private_field(&self) -> &str {
            &self.private_field // 프라이빗 필드 반환
        }
    }

    // 프라이빗 함수 정의
    fn private_function() {
        println!("프라이빗 함수");
    }

    // 퍼블릭 함수 정의
    pub fn public_function() {
        println!("퍼블릭 함수");
        private_function();
    }
}

pub fn main() {
    // 퍼블릭 함수 호출
    access_control::public_function();

    // 퍼블릭 구조체 인스턴스 생성
    let instance = access_control::PublicStruct::new(10, "비공개");

    // 퍼블릭 필드 접근
    println!("퍼블릭 필드 값: {}", instance.public_field);

    // 프라이빗 필드는 직접 접근 불가
    // println!("프라이빗 필드 값: {}", instance.private_field); // 컴파일 에러

    // 프라이빗 필드에 접근하는 퍼블릭 메서드 호출
    println!("프라이빗 필드 값: {}", instance.get_private_field());
}
