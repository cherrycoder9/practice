pub fn immutable_and_mutable_references() {
    // 불변 참조는 참조된 값을 변경할 수 없는 참조
    // 읽기 전용
    let x = 10;

    // 불변 참조 생성
    let ir1 = &x;
    let ir2 = &x;

    // r1과 r2를 통해 x의 값을 읽을 수 있음
    // 여러 개의 불변 참조가 동시에 존재해도 안전함
    println!("ir1: {}", ir1);
    println!("ir2: {}", ir2);

    // *r1 =20; // 컴파일 에러, 불변 참조를 통해 값 변경 불가능

    // 가변 참조
    // 참조된 값을 변경할 수 있는 참조, 값 수정 가능
    let mut y = 20;

    // 가변 참조 생성
    let mr1 = &mut y;
    println!("mr1: {}", mr1);

    // 가변 참조를 통해 y의 값 변경 가능
    *mr1 += 10;

    println!("mr1: {}", mr1);
}
