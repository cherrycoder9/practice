package main

import "fmt"

// 이름 있는 반환값을 사용한 함수
func rectangleArea(width, height int) (area int) {
	area = width * height
	return // return area와 동일
}

func Zero2() {
	fmt.Println(rectangleArea(5, 10)) // 50
}
