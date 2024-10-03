mod result_example;

fn main() {
    /*
       # 에러 처리
       - 에러 처리는 프로그램의 안정성과 신뢰성을 높이는 중요한 메커니즘
       - 러스트에서 에러는 크게 복구 가능한 에러와 복구 불가능한 에러로 구분됨
       * 복구 가능한 에러
       - 프로그램이 에러를 처리하고 계속 실행될 수 있는 상황
       - 파일을 열 때 파일이 존재하지 않으면새로 생성하거나 다른 파일을 시도할 수 있음
       * 복구 불가능한 에러
       - 프로그램이 더 이상 정상적으로 실행될 수 없는 심각한 상황
       - 잘못된 메모리 접근이나 내부 상태의 불일치 등
    */
    /*
       # Result 타입
       - 복구 가능한 에러를 처리하기 위해 사용함
       - 성공과 실패를 나타내는 두 가지 상태인 Ok와 Err을 가짐
    */
    result_example::main();
}
