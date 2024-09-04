package main

import "fmt"

// 여러 개의 정수를 받아 합을 반환하는 가변 인수 함수
func sum(nums ...int) int {
	total := 0
	for _, num := range nums {
		total += num
	}
	return total
}

func Zero4() {
	fmt.Println(sum(1, 2, 3, 4, 5)) // 15
}
