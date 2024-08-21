// 열거형은 서로 관련된 여러 값을 하나의 타입으로 묶는데 사용하는 자료형
// 값의 종류를 명확하게 구분하고 각 값에 대해 다른 데이터 구조를 지정할 수 있음
// 열거형 정의
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn ex_enum() {
    // 열거형 값 사용
    let direction = Direction::North;

    // match 문으로 열거형 값 처리
    match direction {
        Direction::North => println!("북쪽으로 이동"),
        Direction::South => println!("남쪽으로 이동"),
        Direction::East => println!("동쪽으로 이동"),
        Direction::West => println!("서쪽으로 이동"),
    }
}
