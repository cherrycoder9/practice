package main

import "fmt"

// 변수 x를 포인터로 전달받아 값을 2배로 증가시키는 함수 작성

func DoubleValue(x *int) {
	// *x는 포인터 x가 가리키는 실제 값에 접근하는 것을 의미함
	// x의 값을 2배로 증가시킴
	*x = *x * 2
}

func Zero7() {
	// 변수 x를 정수형으로 선언하고 10으로 초기화
	x := 10

	// DoubleValue 함수를 호출해 x의 값을 2배로 만듬
	// 여기서 &x는 변수 x의 메모리 주소를 전달함
	DoubleValue(&x)

	// 함수 호출 후의 x값을 출력
	fmt.Println("첫 번째 호출후 x의 값: ", x) // 20

	// y 변수를 새로운 변수로 선언하고, x의 현재 값을 복사함
	y := x

	// DoubleValue 함수를 다시 호출해 y 값을 2배로 만듬
	DoubleValue(&y)

	// 함수 호출 후의 y값을 출력
	fmt.Println("두 번째 호출후 y의 값: ", y) // 40

	// 함수 호출 후의 x 값을 다시 출력
	// x의 값은 두 번째 호출에 영향을 받지 않으므로 20이어야 함
	fmt.Println("두 번째 호출후 x의 값: ", x)
}
