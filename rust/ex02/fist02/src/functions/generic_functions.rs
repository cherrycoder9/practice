/*
    # 제네릭 함수
    - 다양한 데이터 타입에 대해 유연하게 동작할 수 있도록 타입 매개변수를 사용하는 함수
    - 코드의 재사용성을 높이고, 타입 안정성을 유지함
    - 제네릭은 함수뿐만 아니라 구조체, 열거형 등에서도 사용 가능
    - 트레이트와 함께 사용하면 더 강력한 추상화 구현 가능
*/

// 제네릭 함수 정의
// 타입 매개변수 T가 PartialOrd 트레이트를 구현해야 함을 명시
// list가 T타입의 요소들을 담고 있는 슬라이스
// PartialOrd: 표준 라이브러리에 정의된 트레이트
// 값들 간의 부분적인 순서를 비교할 수 있도록 하는 기능 제공 
// 이 트레이트를 구현함으로써 해당 타입의 값들 사이에 비교 연산자를 사용할 수 있게 됨
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // 리스트의 첫 번째 요소에 대한 참조로 초기화
    let mut largest = &list[0];

    // 리스트를 순회하며 가장 큰 값을 찾음
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("가장 큰 문자: {}", result);
}
