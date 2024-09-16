package main

import (
	"fmt"
	"time"
)

// 채널은 버퍼링 여부에 따라 두 가지 유형이 있음
// 송신과 수신이 동기적으로 일어나는 버퍼링되지 않은 채널
// 송신자가 버퍼에 데이터를 쌓을 수 있어 비동기적 통신이 가능한 버퍼링된 채널

// 데이터 전송 함수
func sendData(ch chan string) {
	fmt.Println("데이터 전송: 시작")
	ch <- "Hello, Channel!"
	fmt.Println("데이터 전송: 완료")
}

// 데이터 수신 함수
func receiveData(ch chan string) {
	time.Sleep(10 * time.Millisecond) // 수신 대기 시간
	data := <-ch
	fmt.Printf("데이터 수신: %s\n", data)
}

func Zero3() {
	fmt.Println("====================")
	// 버퍼링되지 않은 채널 생성
	unbuffered := make(chan string)

	go sendData(unbuffered)
	go receiveData(unbuffered)

	time.Sleep(20 * time.Millisecond)
	fmt.Println("메인 고루틴 종료")

	fmt.Println("====================")

	// 버퍼링된 채널 생성 (버퍼 크기 1)
	buffered := make(chan string, 1)

	go sendData(buffered)
	go receiveData(buffered)

	time.Sleep(20 * time.Millisecond)
	fmt.Println("메인 고루틴 종료")
	fmt.Println("====================")
}
