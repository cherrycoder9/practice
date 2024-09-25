// 타입 별칭은 기존 타입에 새로운 이름을 부여하는 기능
// 복잡한 타입을 간결하게 표현할 수 있게 함
// type 키워드를 사용해 타입 별칭 정의
// 복잡한 제네릭 타입이나 반복적으로 사용되는 타입에 유용

// 타입 별칭 정의
type Kilometers = i32;
type Point = (f64, f64, f64);

// 타입 별칭을 사용하는 구조체 정의
struct Location {
    name: String,
    coordinates: Point,
}

pub fn main() {
    // Kilometers 타입 별칭 사용
    let distance: Kilometers = 15;
    println!("거리는 {} 킬로미터입니다.", distance);

    // Point 타입 별칭 사용
    let origin: Point = (0.0, 0.0, 0.0);
    println!(
        "원점의 좌표는 ({}, {}, {})입니다.",
        origin.0, origin.1, origin.2
    );

    // 구조체에서 타입 별칭 사용
    let location = Location {
        name: String::from("서울"),
        coordinates: origin,
    };
    println!(
        "도시: {}, 위치: ({}, {}, {})",
        location.name, location.coordinates.0, location.coordinates.1, location.coordinates.2
    );
}
