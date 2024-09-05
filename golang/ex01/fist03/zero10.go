package main

import (
	"fmt"
	"math"
)

// 인터페이스를 활용한 폴리모피즘
// Shape 인터페이스를 정의, Area 메서드 포함
// Circle, Rectangle, Triangle 구조체 정의, Area 메서드 구현
// Shape 인터페이스 활용해 PrintArea 함수 작성

// Shape 인터페이스 정의
// 모든 도형은 자신의 넓이를 반환하는 메서드를 구현해야 함
type Shape2 interface {
	Area() float64
}

// Circle 구조체 정의
type Circle2 struct {
	radius float64
}

// Circle 구조체에 대한 Area() 메서드 구현
func (c Circle2) Area() float64 {
	return math.Pi * c.radius * c.radius
}

// Rectangle 구조체 정의
type Rectangle3 struct {
	width, height float64
}

// Rectangle 구조체에 대한 Area() 메서드 구현
func (r Rectangle3) Area() float64 {
	return r.width * r.height
}

// Triangle 구조체 정의
type Triangle2 struct {
	base, height float64
}

// Triangle 구조체에 대한 Area() 메서드 구현
func (t Triangle2) Area() float64 {
	return (t.base * t.height) / 2
}

// PrintArea 함수 정의
// 이 함수는 Shape 인터페이스 타입을 인수로 받음, 해당 도형 넓이를 계산해 출력
// 도형의 타입은 Shape 인터페이스를 통해 추상화되어 있음
func PrintArea(s Shape2) {
	fmt.Printf("도형의 넓이: %.2f\n", s.Area())
}

func Zero10() {
	// Circle 구조체의 인스턴스 생성, 반지름 값은 5.0
	c := Circle2{radius: 5.0}
	// Rectangle 구조체 인스턴스 생성
	r := Rectangle3{width: 4.0, height: 6.0}
	// Triangle 구조체 인스턴스 생성
	t := Triangle2{base: 3.0, height: 4.0}

	// 각 도형 넓이를 PrintArea 함수를 통해 출력
	fmt.Println("원 넓이: ")
	PrintArea(c)

	fmt.Println("사각형 넓이: ")
	PrintArea(r)

	fmt.Println("삼각형 넓이: ")
	PrintArea(t)
}
