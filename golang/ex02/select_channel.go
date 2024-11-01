package main

import (
	"fmt"
	"time"
)

func selectChannel() {
	// 채널 두 개 생성
	messageChannel := make(chan string)
	timeoutChannel := make(chan bool)

	// 첫 번째 고루틴, 메시지를 일정 시간 후에 채널로 송신
	go func() {
		time.Sleep(2 * time.Second) // 2초 대기 후 메시지 전송
		messageChannel <- "메시지 도착"
	}()

	// 두 번째 고루틴, 타임아웃 신호를 일정 시간 후에 채널로 송신
	go func() {
		time.Sleep(3 * time.Second) // 3초 대기 후 타임아웃 신호 전송
		timeoutChannel <- true
	}()

	// select 문을 사용해 여러 채널의 데이터를 기다림
	for {
		select {
		// 메시지 채널에서 데이터 수신 시 처리
		case msg := <-messageChannel:
			fmt.Println("수신된 메시지:", msg)
			return // 메시지를 수신하면 프로그램 종료

		// 타임아웃 채널에서 신호 수신 시 처리
		case <-timeoutChannel:
			fmt.Println("타임아웃 발생! 프로그램 종료")
			return // 타임아웃시 프로그램 종료

			// 아무런 채널도 준비되지 않은 경우
		default:
			fmt.Println("대기 중...")
			time.Sleep(500 * time.Millisecond) // 0.5초 대기 후 다시 시도
		}
	}

	// select 문은 여러 채널에서 데이터를 기다릴 때 사용함
	// 각 채널 중 하나가 준비되면 해당 분기를 실행함

}
