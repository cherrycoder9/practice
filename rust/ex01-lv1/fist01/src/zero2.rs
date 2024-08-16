pub fn mutable_and_immutable_variables() {
    // 주어진 x, y의 값을 출력하고 x를 가변 변수로 바꾼후
    // x의 값을 10으로 변경한 후 다시 출력
    let x = 5;
    let y = 15;
    println!("x: {}", x);
    println!("y: {}", y);

    let mut x = x;
    x = 10;
    println!("x: {}", x);
    println!("y: {}", y);
}
