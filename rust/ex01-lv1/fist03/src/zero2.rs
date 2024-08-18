// 숫자 판단
// 숫자 n이 주어졌을때, 이 숫자가 0보다 큰지,
// 0과 같은지, 또는 0보다 작은지 판별해 각 경우에 맞는 메시지 출력
pub fn check_number(n: i32) {
    if n > 0 {
        println!("0보다 큼")
    } else if n < 0 {
        println!("0보다 작음")
    } else {
        println!("0임")
    }
}
