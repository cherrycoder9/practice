// crate:: 는 현재 프로젝트의 루트 모듈을 가리킴
// use는 이미 선언된 모듈을 가져와서 사용할때 쓰임
use fist02::functions::function_parameters;
use fist02::functions::function_return;
fn main() {
    function_parameters::main();
    function_return::main();
}
