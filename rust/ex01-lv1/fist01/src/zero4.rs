pub fn variable_shadowing() {
    // 변수 text 선언 및 숫자 10을 할당하고 출력한 다음,
    // 동일한 변수 이름을 사용해 문자열 Rust를 할당한 후 다시 출력
    let text = 10;
    println!("text: {}", text);

    let text = "Rust";
    println!("text: {}", text);
}
