mod network;

fn main() {
    /*
       # 네임스페이스 중첩과 계층 구조
       - 중첩된 네임스페이스는 모듈을 계층적으로 구성해 관련 기능을 그룹화함
       - 코드의 논리적 구조를 명확히 하고, 복잡성을 줄일 수 있음
    */
    network::server::start_server();
    network::client::start_client();
}
