mod lifetime_syntax;
mod structs_with_lifetimes;
mod understanding_lifetimes;

fn main() {
    understanding_lifetimes::main();
    lifetime_syntax::main();

    // 구조체에서 참조를 포함할 때는 해당 참조의 수명을 구조체에 명시적으로 지정해야 함
    // 구조체 인스턴스가 참조하는 데이터의 수명이 구조체보다 짧지 않음을 보장할 수 있음
    structs_with_lifetimes::main();
}
