/// Option과 Result 타입 차이점을 보여주는 예제 함수
// usize는 플랫폼에 따라 크기가 결정되는 부호 없는 정수형 자료형
// 주로 인덱스, 메모리 크기, 포인터 연산 등에 사용됨
fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    // get 메서드는 벡터에서 특정 인덱스에 위치한 요소에 안전하게 접근할때 사용
    // 인덱스가 유효한 경우 Some(&T)를 반환, &T는 해당 요소에 대한 참조임
    // copied 메서드는 Option<&T> 타입을 Option<T> 타입으로 변환함
    // 여기서 T는 Copy 트레이트를 구현해야 함. 대부분의 기본 타입들은 이미 구현되어 있음
    vec.get(index).copied() // 인덱스가 유효하면 Some 값, 아니면 None 반환
}

fn get_element_result(vec: &Vec<i32>, index: usize) -> Result<i32, String> {
    if index < vec.len() {
        Ok(vec[index]) // 인덱스가 유효하면 Ok 값 반환
    } else {
        Err(String::from("인덱스가 범위를 벗어났습니다.")) // 유효하지 않으면 에러 반환
    }
}

pub fn main() {
    let numbers = vec![10, 20, 30];

    // Option 타입 사용
    match get_element(&numbers, 1) {
        Some(num) => println!("인덱스 1의 값: {}", num),
        None => println!("인덱스 1에 해당하는 값이 없습니다."),
    }

    // Result 타입 사용
    match get_element_result(&numbers, 5) {
        Ok(num) => println!("인덱스 5의 값: {}", num),
        Err(e) => println!("에러: {}", e),
    }
}
