// Enum-based State Machine
// 간단한 상태 기계를 enum을 사용해 구현
// 상태는 Start, Processing, Completed로 정의됨
// 상태 전환 함수 next_state를 작성해 현재 상태에서 다음 상태로 전환
// 단, Completed 상태에서는 더 이상 전환이 워맂지 않아야 함

#[derive(Debug)]
enum State {
    Start,
    Processing,
    Completed,
}

// self, &self, &mut self의 차이점
// self
// 메서드를 호출할 때, 그 메서드가 호출된 객체의 소유권을 가져감
// 메서드 호출 이후에 해당 객체를 사용할 수 없다는 뜻
// 주로 소유권을 완전히 넘겨주는 경우에 사용
// &self
// 메서드가 객체의 참조를 받음
// 객체의 소유권을 가져가지 않고, 객체를 빌려오기 때문에 메서드가
// 끝난 후에도 원래 객체를 계속 사용할 수 있음
// 주로 객체의 상태를 읽기 전용으로 접근할 때 사용됨
// &mut self
// 메서드가 객체의 가변 참조를 받음
// 객체의 소유권을 가져가지 않지만, 참조를 통해 객체의 상태를 변경할 수 있음
// 주로 객체의 상태를 변경할 때 사용됨

impl State {
    fn next_state(self) -> State {
        match self {
            State::Start => State::Processing,
            State::Processing => State::Completed,
            State::Completed => State::Completed,
        }
    }
}

pub fn main() {
    // :?는 러스트에서 데이터를 디버그 형식으로 출력하기 위해 사용되는 형식 지정자
    // #[derive(Debug)]를 enum위에 적어줘야 함
    let state = State::Start;
    println!("현재상태: {:?}", state);
    let state = state.next_state();
    println!("현재상태: {:?}", state);
    let state = state.next_state();
    println!("현재상태: {:?}", state);
    let state = state.next_state();
    println!("현재상태: {:?}", state);
}
