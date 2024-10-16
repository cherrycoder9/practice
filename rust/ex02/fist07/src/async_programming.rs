use std::time::Duration;
use tokio::time;

// Tokio는 비동기 런타임을 제공해 async 함수를 실행 가능하게 함

// async 키워드를 사용해 비동기 함수 정의
// 해당 함수는 Future를 반환, 호출 즉시 실행되지 않고 await될 때 실행됨
async fn say_hello() {
    // 비동기 함수에서 실제로 비동기적 작업을 수행하려면
    // await 키워드를 통해 비동기적으로 결과를 기다림
    time::sleep(Duration::from_secs(2)).await;
    println!("안녕하세요. 비동기 Rust 함수입니다!"); // 2초 후 메시지 출력
}

// 비동기 함수 간 호출
// async 함수를 다른 async 함수에서 호출할 수 있음
// 이때 호출된 함수도 반드시 await 되어야 함
async fn call_hello_multiple() {
    println!("say_hello 함수를 여러 번 호출합니다");
    say_hello().await; // 첫번째 호출, 2초 대기후 메시지 출력
    say_hello().await; // 두번째 호출, 2초 대기후 메시지 출력
}

// 메인 함수
// Tokio의 #[tokio::main] 어트리뷰트를 사용해 메인 함수를 비동기 컨텍스트에서 실행 가능하게 함
// cargo.toml에 tokio 의존성 추가해야 함
#[tokio::main]
pub async fn main() {
    println!("비동기 메인 함수 시작");

    // 비동기 함수 호출 및 await
    call_hello_multiple().await;

    println!("모든 비동기 작업 완료");
}
