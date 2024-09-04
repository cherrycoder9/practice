package main

import "fmt"

// Go에서 메서드는 특정 타입에 바인딩된 함수
// 메서드를 사용하면 구조체와 같은 사용자 정의 타입에 동작을 정의할 수 있음
// 함수형과 절차지향 프로그래밍을 중시하는 설계 철학에 기인한 것

// 사용자 정의 구조체
type Rectangle struct {
	width, height int
}

// 구조체에 바인딩된 메서드
func (r Rectangle) area() int {
	return r.width * r.height
}

func Zero5() {
	rect := Rectangle{width: 10, height: 5}
	fmt.Println(rect.area()) // 50
}
