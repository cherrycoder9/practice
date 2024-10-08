package main

func Zero1() {
	// 명시적 타입 선언
	// var 키워드는 변수를 선언할 때 사용함
	// 초기화를 하지 않으면 해당 타입의 제로 값이 할당됨
	var x int = 10

	// 암시적 타입 추론
	// := 구문은 초기화와 동시에 변수의 타입을 추론하여 선언하는 방법
	y := 20 // y는 int 타입으로 추론됨

	println(x, y)
}
