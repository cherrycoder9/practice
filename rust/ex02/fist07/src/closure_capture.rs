pub fn main() {
    /*
       # 클로저의 캡처 동작 방식
       - 불변 참조
       - 가변 참조
       - 소유권 이동
    */

    let mut num = 5;

    // 불변 참조 클로저
    let add_to_num = |x: i32| -> i32 {
        // 불변으로 num을 캡처해 클로저 내부에서 사용함
        // num 값을 읽기만 하고 변경하지 않음
        println!("불변 참조 클로저 실행: num = {}", num);
        num + x
    };
    println!("불변 참조 결과: {}", add_to_num(3));

    // 가변 참조 클로저
    let mut modify_num = || {
        // 가변으로 num을 캡처해 클로저 내부에서 수정함
        // num의 값을 변경 가능하도록 빌림
        num += 1;
        println!("가변 참조 클로저 실행 후: num = {}", num);
    };
    modify_num();
    println!("가변 참조 이후의 num 값: {}", num);

    // 소유권 이동 클로저
    let consume_num = move || {
        // move 키워드를 사용해 num의 소유권을 클로저로 이동시킴
        // 클로저 내부에서 num을 사용하고, 이후에는 사용할 수 없음
        println!("소유권 이동 클로저 실행: num = {}", num);
    };
    consume_num();
    // num은 소유권이 이동되었으므로 이 이후로는 사용할 수 없음
}
