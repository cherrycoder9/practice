// main 모듈을 선언하는 주석
mod main_module {
    /// 인라인으로 선언한 모듈 내의 함수
    pub fn greet() {
        println!("메인 모듈입니다.");
    }
}

pub fn main() {
    // main_module의 greet 함수를 호출
    main_module::greet();
}
