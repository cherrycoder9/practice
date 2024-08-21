// 열거형에 데이터 추가하기
// 각 값이 추가적인 데이터를 가질 수 있음
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn ex_enum2() {
    // let msg = Message::Move { x: 10, y: 20 };
    let msg = Message::ChangeColor(100, 200, 50);

    match msg {
        Message::Quit => println!("종료 메시지"),
        Message::Move { x, y } => println!("다음 좌표로 이동, x: {}, y: {}", x, y),
        Message::Write(text) => println!("메시지: {}", text),
        Message::ChangeColor(r, g, b) => println!("색상 변경: 빨강 {}, 초록 {}, 파랑 {}", r, g, b),
    }
}
