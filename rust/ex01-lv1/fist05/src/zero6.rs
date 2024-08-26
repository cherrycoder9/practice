// Complex Enum with Recursive Structures
// 재귀적 enum을 사용해 Rust에서 수식 계산기를 작성
// enum을 사용해 덧셈, 곱셈, 숫자, 그리고 다른 수식을 포함할 수 있는 복합 수식을 표현
// eval_expression 함수를 작성해 주어진 수식의 값을 계산

enum Expression {
    Number(i32),
    // Box<T>는 Rust 표준 라이브러리에서 제공되는 기본 타입 중 하나임
    // 힙(heap)에 데이터를 할당하고, 이 데이터에 대한 소유권을 가질 수 있는 스마트포인터
    // 러스트에서는 기본적으로 모든 값이 스택에 저장되지만
    // Box<T>를 사용하면 값을 힙에 저장할 수 있음. 크기가 큰 데이터를 다루거나
    // 재귀적인 데이터 구조(트리 등)를 처리할때 유용함
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

fn eval_expression(expr: &Expression) -> i32 {
    match expr {
        Expression::Number(value) => *value,
        Expression::Add(left, right) => eval_expression(left) + eval_expression(right),
        Expression::Multiply(left, right) => eval_expression(left) * eval_expression(right),
    }
}

pub fn zero6() {
    // a + (b * c)
    let expr = Expression::Add(
        Box::new(Expression::Number(5)),
        Box::new(Expression::Multiply(
            Box::new(Expression::Number(3)),
            Box::new(Expression::Number(4)),
        )),
    );

    let result = eval_expression(&expr);
    println!("결과: {}", result);
}
