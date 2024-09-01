package main

import "fmt"

// 두 개의 정수를 입력받고, 합과 차를 계산한 뒤, 출력함수로 전달해 출력
// 함수 내에서 일부 블록은 원래의 두 변수와 다른 값을 가지도록 섀도잉 적용
// 출력은 원래 두 변수의 값을 사용해 계산된 합과 차로 할 것

// 합과 차를 출력하는 함수
func PrintSumAndDifference(sum int, difference int) {
	fmt.Println("합: ", sum)
	fmt.Println("차이: ", difference)
}

// 주어진 두 정수 a, b의 합과 차를 계산해 출력하는 함수
func CalculateSumAndDifference(a int, b int) {
	// 첫 번째 블록
	sum := a + b
	difference := a - b

	// 두 번째 블록
	if true {
		a := 100
		b := 50
		sum = a + b
	}

	PrintSumAndDifference(sum, difference)
}

func Zero6() {
	a := 7
	b := 3

	CalculateSumAndDifference(a, b)
}
