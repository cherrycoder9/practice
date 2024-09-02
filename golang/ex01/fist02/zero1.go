package main

import "fmt"

func Zero1() {
	x := 10

	// x 값에 따라 다른 출력
	if x > 10 {
		fmt.Println("x는 10보다 큼")
	} else if x == 10 {
		fmt.Println("x는 10")
	} else {
		fmt.Println("x는 10보다 작음")
	}

	// if문 안에서 초기화 구문 사용하기
	if y := 20; y > x {
		fmt.Println("y는 x보다 큼")
	}
}
