// 구조체 기본 정의
// 여러개의 필드를 하나의 타입으로 묶어주는 역할
// 관련 있는 데이터를 함께 묶어서 사용하기에 용이함
// struct 키워드를 사용해 정의함
pub struct Person {
    // main.rs에서 사용하기 위해 구조체와 필드 앞에 pub 붙임
    pub name: String,
    pub age: u32,
    pub email: String,
}
