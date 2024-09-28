/*

    # 반환 값과 return 키워드
    - return 키워드를 사용해 명시적으로 값 반환 가능
    - 마지막 표현식을 세미콜론 없이 작성하면 암묵적으로 반환됨
    - 반환값은 함수의 소유권 규칙에 따라 호출자에게 이전됨
*/

pub fn main() {
    let number = 10;

    let square = calculate_square(number);

    println!("{}의 제곱은 {}", number, square);
}

// 정수를 받아 그 제곱을 반환
fn calculate_square(x: i32) -> i32 {
    return x * x;
}
