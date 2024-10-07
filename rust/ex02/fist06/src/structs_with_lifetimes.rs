pub fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world");

    let s1 = MyStruct { part: &string1 };
    let s2 = MyStruct { part: &string2 };

    println!("s1.part: {}", s1.part);
    println!("s2.part: {}", s2.part);
}

/// MyStruct는 part 필드에 대한 참조를 포함하며, 'a 수명을 가짐
/// 수명이 명시되지 않으면 러스트 컴파일러는 참조의 수명을 알 수 없어 컴파일 오류 발생
struct MyStruct<'a> {
    part: &'a str,
}
