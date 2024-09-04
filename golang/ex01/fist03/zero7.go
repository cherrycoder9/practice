package main

import "fmt"

// 인터페이스는 메서드의 집합을 정의함
// 특정 타입이 인터페이스를 구현하려면 해당 인터페이스에 정의된 모든 메서드를 구현해야 함

// 인터페이스 정의
type Shape interface {
	area() int
}

// Rectangle 구조체 정의
type Rectangle2 struct {
	width, height int
}

// Rectangle 구조체가 Shape 인터페이스를 구현
func (r Rectangle2) area() int {
	return r.width * r.height
}

// Circle 구조체 정의
type Circle struct {
	radius int
}

// Circle 구조체가 Shape 인터페이스를 구현
func (c Circle) area() int {
	return 3 * c.radius * c.radius // pi는 3으로 처리함
}

func Zero7() {
	var s Shape
	s = Rectangle2{width: 10, height: 5}
	fmt.Println(s.area()) // 50

	s = Circle{radius: 3}
	fmt.Println(s.area()) // 27
}
