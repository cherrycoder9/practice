pub fn main() {
    // 외부 스코프에서 불변 변수 선언 및 초기화
    let external_var = 20;
    println!("외부 스코프의 불변 변수 초기값: {}", external_var);

    // 내부 스코프 시작
    {
        // 외부 변수와 동일한 이름의 가변 변수 선언
        let mut external_var = external_var;
        // 외부 변수의 값을 가변 변수로 복사
        println!("내부 스코프 변수 값: {}", external_var);

        // 가변 변수의 값 변경
        external_var = 30;
        println!("내부 스코프에서 변경된 변수 값: {}", external_var);
    }
    // 내부 스코프 종료, 가변 변수는 소멸됨

    // 외부 스코프로 돌아와 불변 변수의 값 출력
    println!("외부 스코프로 돌아온 불변 변수 값: {}", external_var);
}
