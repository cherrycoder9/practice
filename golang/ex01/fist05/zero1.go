package main

import (
	"fmt"
	"time"
)

// 고루틴은 Go 언어에서 경량 스레드로 동시성을 구현하는 기본 단위
// 비동기적으로 함수를 실행해 프로그램 응답성을 높이고, 병렬 처리를 용이하게 함

func sayHello(id int) {
	for i := 0; i < 5; i++ {
		fmt.Printf("고루틴 %d: 안녕! (%d)\n", id, i)
		time.Sleep(100 * time.Millisecond)
	}
}

func Zero1() {
	go sayHello(1) // 고루틴 1 실행
	go sayHello(2) // 고루틴 2 실행

	// 메인 고루틴이 종료되지 않도록 대기
	time.Sleep(1 * time.Second)
	fmt.Println("메인 고루틴 종료")
}
