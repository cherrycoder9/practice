package main

import "fmt"

// 원 반지름을 입력받아 원의 둘레와 면적을 계산해 반환하는 함수 구현
// 이 함수는 반드시 상수를 이용해 계산을 해야 함

// 상수 Pi와 Two 선언
const Pi = 3.141592653589793 // 원주율
const Two = 2                // 둘레 계산에 사용되는 상수

// CalculateCircleProperties 함수는 원의 반지름을 입력받아, 원 둘레와 면적을 계산해 반환
func CalculateCircleProperties(radius float64) (circumference float64, area float64) {
	// 원 둘레 계산
	circumference = float64(Two) * Pi * radius

	// 원 면적 계산
	area = Pi * radius * radius

	// 둘레와 면적을 반환
	// 반환 타입이 각각 float64로 지정되어 있기 때문에 자동으로 타입 일치가 이뤄짐
	return
}

func Zero5() {
	// 반지름을 입력받음
	var radius float64
	fmt.Print("반지름 입력: ")
	fmt.Scanf("%f", &radius)

	// 함수 호출하고 둘레와 면적 계산
	circumference, area := CalculateCircleProperties(radius)

	// 출력
	fmt.Printf("원의 둘레: %.2f\n", circumference) // 소수점 둘째 자리까지 출력
	fmt.Printf("원의 면적: %.2f\n", area)          // 소수점 둘째 자리까지 출력
}
