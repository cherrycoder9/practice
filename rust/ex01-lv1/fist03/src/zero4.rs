// 숫자 카운트다운
// 정수 start가 주어졌을때, 1까지 감소하는 카운트다운을 수행하고
// 마지막에 Start!를 출력하는 함수 작성, while 반복문 사용
pub fn countdown(start: i32) {
    let mut i = start;
    while i != 0 {
        // 러스트에서는 i++, i-- 같은 연산자 사용 불가함
        // i--;
        println!("{}", i);
        i -= 1;
    }
    println!("Start!");
}
