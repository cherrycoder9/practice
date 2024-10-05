pub fn main() {
    let r; // r의 수명 시작
    {
        let x = 5; // x의 수명 시작
        r = &x; // r이 x를 참조
                // x의 수명 종료
        println!("r: {}", r);
    }
    // r은 더 이상 유효하지 않은 x를 참조하게 되어 컴파일 에러 발생
    // println!("r: {}", r);
}
