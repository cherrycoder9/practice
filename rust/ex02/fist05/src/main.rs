mod custom_error;
mod implement_error_trait;
mod option_example;
mod option_vs_result;
mod propagate_error;
mod result_example;
mod result_generic;
mod using_custom_error;

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

    /*
       # Result의 제네릭 타입
       - Result는 제네릭 타입을 사용해 다양한 데이터 타입과 에러 타입 처리 가능
       - 일반적인 형식은 Result<T, E>로, T는 성공시 반환되는 값의 타입, E는 에러의 타입
    */
    result_generic::main();

    /*
       # Option 타입과 Result 타입 비교
       - 두 타입 모두 열거형으로, 값의 유무나 연산의 성공 여부를 나타내는데 사용됨
       - Option: 값이 있을 수도 있고 없을 수도 있는 상황을 나타냄
       - 실패의 원인을 명확히 알 필요가 없거나, 단순히 값의 유무만을 확인할 때 사용함
       - Result: 연산의 성공 여부와 에러의 원인을 명확히 알 필요가 있는 상황에 사용
       - 에러의 원인을 자세히 처리하거나 전달해야 할 때 적합
    */
    option_example::main();
    option_vs_result::main();

    /*
       # 에러 전파와 ? 연산자
       - 함수가 실행 중에 에러를 만났을 때, 그 에러를 호출한 쪽으로 전달하는 과정
       - ? 연산자는 Result 타입을 반환하는 함수에서 에러를 간편하게 전파할 수 있는 문법적 편의성
       - ?를 사용하면 에러 발생시 즉시 함수가 종료되고, 에러가 호출자에게 전달됨
    */
    propagate_error::main();

    /*
      # 커스텀 에러 타입 정의
      - 프로젝트의 도메인에 특화된 에러 처리를 위함
      - struct나 enum을 사용해 에러 타입을 설계하고, std::error::Error 트레이트와
      - Display, Debug 트레이트를 구현해 에러 가독성 및 디버깅 정보 향상
    */
    custom_error::main();
    implement_error_trait::main();
    using_custom_error::main();
}
