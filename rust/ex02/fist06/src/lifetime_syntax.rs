pub fn main() {
    /*
       # 수명 표기법
       - 수명을 명시적으로 지정할 땐 수명 매개변수를 사용함 ('a, 'b 등)
       - 함수 시그니처나 구조체 정의에서 참조의 수명을 명확하게 하기 위해 사용
    */
    let string1 = String::from("hello");
    let string2 = String::from("world");
    let result = longest(&string1, &string2);
    println!("가장 긴 문자열은: {}", result);
}

/// 두 문자열 슬라이스 중 더 긴 것을 반환하는 함수
/// 'a는 두 매개변수와 반환값의 수명을 연결짓는 수명 매개변수임
/// 입력 참조 x와 y, 그리고 반환 참조의 수명이 모두 동일함을 나타냄
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
