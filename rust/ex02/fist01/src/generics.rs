// 제네릭 타입은 코드 재사용성을 높이고 다양한 타입에 유연하게 동작할 수 있도록 함
// 함수, 구조체, 열거형에 적용할 수 있음. 컴파일 시 구체적인 타입으로 대체됨
// 타입 안정성을 유지하면서 코드중복을 줄일 수 있음

// 제네릭 타입을 사용하는 스택 구조체 정의
struct Stack<T> {
    elements: Vec<T>, // 스택의 요소를 저장하는 벡터
}

impl<T> Stack<T> {
    // 새로운 빈 스택을 생성하는 연관 함수
    fn new() -> Stack<T> {
        Stack {
            elements: Vec::new(),
        }
    }

    // 스택에 요소를 추가하는 메서드
    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    // 스택에서 마지막 요소를 제거하고 반환하는 메서드
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // 스택의 현재 요소 개수를 반환하는 메서드
    fn size(&self) -> usize {
        self.elements.len()
    }
}

pub fn main() {
    // 정수형 스택 생성
    // :: 연산자는 범위 연산자라고도 부르며 여러 용도가 있음
    // 1. 모듈 및 네임스페이스 접근, std::io::Write는 std모듈 내의 io 모듈에 있는 Write 트레이트
    // 2. 연관 함수 및 메서드 호출, 아래에서 사용된 new
    // 3. 타입 별칭 및 제네릭타입 사용, Option::Some, Result::Ok
    let mut int_stack = Stack::new();
    int_stack.push(10);
    int_stack.push(20);
    println!("정수 스택의 크기: {}", int_stack.size());
    int_stack.pop();
    println!("정수 스택의 크기: {}", int_stack.size());

    // 문자열형 스택 생성
    let mut string_stack = Stack::new();
    string_stack.push(String::from("Hello"));
    string_stack.push(String::from("World"));
    println!("문자열 스택 크기: {}", string_stack.size());
    string_stack.pop();
    println!("문자열 스택 크기: {}", string_stack.size());
}
