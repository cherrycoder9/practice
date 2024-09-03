package main

import "fmt"

// 다중 반환값 함수
// 두 개의 정수를 더한 값과 차이를 반환하는 함수
func addAndSubtract(x, y int) (int, int) {
	return x + y, x - y
}

func Zero3() {
	sum, diff := addAndSubtract(10, 3)
	fmt.Println(sum, diff) // 13 7
}
