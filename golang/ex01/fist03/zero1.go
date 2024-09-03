package main

import "fmt"

// 두 개의 정수를 더하는 함수
func add(x int, y int) int {
	return x + y
}

func Zero1() {
	result := add(3, 4)
	fmt.Println(result) // 7
}
