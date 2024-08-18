pub fn if_else() {
    let number = 5;

    // 조건이 참일 때 실행
    if number < 10 {
        println!("10보다 작음");
    } else if number == 10 {
        println!("10임");
    } else {
        println!("10보다 큼");
    }

    // 조건의 결과를 변수에 할당할 수 있음
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);
}

pub fn ex_match() {
    let number = 3;

    // match로 여러 경우 처리
    match number {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("다른 숫자"), // 와일드카드 패턴
    }

    // Option 타입과 match
    // Option 타입은 값이 있을수도 있고 없을 수도 있는 상황을 표현할때 사용
    // null 값의 사용을 피하면서 안전하게 값의 유무를 처리하는 기능
    // Option 타입은 열거형(enum)으로 정의되고 두가지 변형을 가질 수 있음
    // Some(T): 값이 있을 때 사용됨. T는 값의 타입
    // None: 값이 없을 때 사용됨
    // 값이 있는지 없는지 확인하기 위해 match 구문을 자주 사용함
    let some_number = Some(5);
    match some_number {
        Some(5) => println!("숫자 5"),
        Some(_) => println!("다른 숫자"),
        None => println!("숫자가 없음"),
    }
    // 아래처럼 표현할 수도 있음
    match some_number {
        Some(n) if n > 3 => println!("숫자 {}는 3보다 큼", n),
        Some(n) => println!("숫자 {}는 3 이하", n),
        None => println!("숫자가 없음"),
    }
}

pub fn ex_while() {
    let mut number = 3;

    // 조건이 참인 동안 반복
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("출발!");
}

pub fn ex_for() {
    let a = [10, 20, 30, 40, 50];

    // 배열의 각 요소에 대해 반복
    for element in a.iter() {
        println!("값: {}", element);
    }

    // 범위를 지정해 반복
    for number in 1..4 {
        println!("{}!", number);
    }
    println!("출발!");
}

pub fn ex_loop() {
    let mut count = 0;

    // 무한 반복, while과 같음
    loop {
        count += 1;
        println!("카운트: {}", count);

        if count == 3 {
            break; // 반복 종료
        }
    }
    println!("루프 종료");

    // 값을 반환하는 형태
    let result = loop {
        let condition = true;
        if condition {
            break 100;
        }
    };
    println!("result: {}", result);
}
