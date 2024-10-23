package variables_and_data_types

import "fmt"

func typeInference() {
	x := 10
	y := 20.5
	message := "타입 추론"

	// %T는 변수의 타입 출력
	fmt.Printf("x 값: %d, 타입: %T\n", x, x)
	fmt.Printf("y 값: %f, 타입: %T\n", y, y)
	fmt.Printf("message 값: %s, 타입: %T\n", message, message)

	// 배열과 슬라이스도 타입 추론 가능
	// 배열은 크기가 고정되어 있고 선언시 크기를 지정해야 하며 크기 변경 불가
	// var arr [5]int
	// 슬라이스는 크기가 가변적이고 요소 추가나 제거 가능함
	// var slice []int
	numbers := []int{1, 2, 3, 4, 5} // numbers는 슬라이스로 추론됨 ([]int)
	fmt.Printf("numbers 타입: %T\n", numbers)

	// 함수 내에서도 타입 추론 가능
	result := add(3, 4) // 반환값을 추론해 result 변수에 할당
	fmt.Printf("result의 값: %d, 타입: %T\n", result, result)
}

func add(a int, b int) int {
	return a + b
}
