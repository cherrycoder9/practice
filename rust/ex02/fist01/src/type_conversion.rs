// 타입 변환은 보통 to_string(), parse() 등의 메서드 사용
// 캐스팅은 as 키워드를 사용해 기본 타입 간의 변환 수행
// 타입 변환은 한 타입의 값을 다른 타입으로 변환하는 메서드를 호출하는 방식
// 캐스팅은 주로 숫자 타입 간의 변환에 사용됨, 캐스팅은 데이터 손실이나 오버플로우 주의

pub fn main() {
    // 문자열을 숫자로 변환
    let num_str = "42";
    let num: i32 = num_str.parse().expect("유효한 숫자가 아님");
    println!("num_str을 i32로 변환한 값: {}", num);

    // 숫자를 문자열로 변환
    let num = 100;
    let num_string = num.to_string();
    println!("num을 문자열로 변환한 값: {}", num_string);

    // 캐스팅을 사용한 숫자 타입 변환
    let a: u8 = 10;
    let b: u16 = a as u16;
    println!("u8 타입 a의 값: {}, u16 타입 b의 값: {}", a, b);

    // 캐스팅 시 데이터 손실
    let large_num: u16 = 1000;
    let small_num = large_num as u8;
    println!(
        "u16 타입 large_num의 값: {}, u8 타입으로 캐스팅한 small_num의 값: {}",
        large_num, small_num
    );
}
