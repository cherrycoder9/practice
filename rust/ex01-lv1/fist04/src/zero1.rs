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

impl Person {
    // 러스트에서 self는 자바에서 this와 유사한 역할을 함
    // 키워드가 아닌 식별자이므로 this: &self 처럼 이름을 바꿔 사용할수 있지만
    // 관례는 self를 그대로 사용하는 것임
    // 또한 self는 첫 번째 매개변수로 사용하는게 일반적 관례
    pub fn greeting(&self) -> String {
        // format!은 String 타입의 문자열을 반환함, 형식화된 문자열 생성 함수
        // 문자열을 동적으로 생성하고 반환해야 할 때 유용함.
        format!("안녕하세요, {}입니다.", self.name)
    }
}
