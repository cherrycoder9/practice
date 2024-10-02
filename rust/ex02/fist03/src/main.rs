mod access_control;
mod external_module;
mod inline_module;
mod parent_module;
mod re_export;
mod use_namespace;
mod utils;

fn main() {
    /*
       # 모듈의 목적과 필요성
       - 관련된 함수, 구조체, 열거형 등을 하나의 모듈로 그룹화해 코드 구조를 명확하게 함
       - 모듈을 통해 동일한 이름의 항목들이 서로 다른 네임스페이스에 존재할 수 있도록해 이름충돌 방지
       - 모듈화된 코드는 재사용이 용이하며, 변경 사항 발생시 영향을 받는 범위를 최소화함
    */
    inline_module::main();
    external_module::greet();
    // parent_module의 child_module의 greet 함수를 호출
    parent_module::child_module::greet();

    /*
       # 모듈 트리와 파일 시스템 구조
       - 러스트의 모듈 트리는 계층적 구조로 이뤄짐
       - 루트 모듈은 main.rs 또는 lib.rs 파일
       - 서브모듈은 디렉터리와 파일로 구성됨
       src/
       ├── main.rs          // 루트 모듈
       └── utils/
           ├── mod.rs       // utils 모듈
           ├── math.rs      // math 서브모듈
           └── string.rs    // string 서브모듈
    */
    // utils 모듈의 math 서브모듈의 add 함수 호출
    let sum = utils::math::add(5, 3);
    println!("합계: {}", sum);

    /*
       # 절대 경로와 상대 경로
       - 절대경로: 프로젝트의 루트(crate)를 기준으로 경로를 지정
       - 상대경로: 현재 모듈을 기준으로 경로를 지정
    */
    // 절대 경로를 사용해 math 모듈의 add 함수 호출
    let sum = crate::utils::math::add(10, 20);
    println!("합계: {}", sum);
    // 상대 경로 예시는 math.rs 확인

    /*
       # 접근 제어 및 캡슐화
       - 러스트에선 모듈 내에서의 접근을 제어하기 위해 pub 키워드 사용
       - 기본적으로 모든 항목은 프라이빗(private)으로 설정됨.
       - 외부에서 접근하려면 pub 키워드를 통해 공개 설정을 해야 함
    */
    access_control::main();

    /*
       # use 키워드를 통한 네임스페이스 관리
       - use 키워드는 모듈의 경로를 간소화해 코드 내에서 네임스페이스를 효과적으로 관리
       - 긴 경로를 반복적으로 작성하지 않고도 모듈 내의 항목에 접근할 수 있음
       - 가져온 항목은 현재 스코프 내에서 직접 사용할 수 있게 되며 경로생략 가능
       - use 키워드와 함께 as 키워드를 사용해 가져온 항목에 별칭 지정 가능
       - 별칭은 네임스페이스 충돌을 방지하거나, 더 명확한 이름을 사용할 때 씀
    */
    use_namespace::main();

    /*
       # 재수출과 pub use
       - 재수출은 모듈 내부에서 가져온 항목을 외부로 다시 공개하는 것
       - 모듈의 인터페이스를 단순화하고, 외부에서 필요한 항목만 노출
    */
    re_export::main();
}
