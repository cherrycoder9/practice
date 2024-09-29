pub fn main() {
    let mut s = String::from("Rust");

    // 불변 참조 생성
    let r1 = &s;
    let r2 = &s;
    println!("불변 참조 r1: {}", r1);
    println!("불변 참조 r2: {}", r2);

    // 가변 참조 생성
    let r3 = &mut s;
    r3.push_str(" 프로그래밍");
    println!("가변 참조 r3: {}", r3);

    // 가변 참조 이후 다시 불변 참조 생성
    // 비지역적 생에 규칙으로 인해 가능함
    let r4 = &s;
    println!("r4: {}", r4);
}
