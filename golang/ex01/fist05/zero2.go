package main

import (
	"fmt"
	"time"
)

// 채널은 고루틴 간에 데이터를 주고받을 수 있는 파이프라인 역할
// 채널은 특정 타입의 데이터만 전송할 수 있음
// 기본적으로 송신은 수신이 이루어질 때까지 block됨
// 채널은 양방향 또는 단방향으로 사용할 수 있음

// 숫자를 전송하는 함수, 채널 타입 선언 chan 데이터타입
// 정수를 주고 받을수 있는 채널
func sendNumbers(ch chan int) {
	for i := 1; i < 15; i++ {
		fmt.Printf("숫자 전송: %d\n", i)
		// <- 연산자는 채널을 통해 데이터를 전송하거나 수신할때 사용함
		// 데이터 전송: ch <- i
		// 데이터 값을 채널로 전송함
		// 데이터 수신: i := <-ch
		// 채널로부터 데이터를 수신해 i에 저장
		ch <- i
		time.Sleep(10 * time.Millisecond)
	}
	close(ch) // 채널 닫기
}

// 숫자를 수신하는 함수
func receiveNumbers(ch chan int) {
	for num := range ch {
		fmt.Printf("숫자 수신: %d\n", num)
	}
}

func Zero2() {
	// 정수형 채널 생성
	numberChannel := make(chan int)

	// 송신 고루틴 실행
	go sendNumbers(numberChannel)

	// 수신 고루틴 실행
	receiveNumbers(numberChannel)

	fmt.Println("메인 고루틴 종료")
}
