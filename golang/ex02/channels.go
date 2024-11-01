package main

import (
	"fmt"
	"time"
)

func channels() {
	// 채널을 사용한 고루틴 간 동시성 처리 예제
	// 비버퍼링 및 버퍼링된 채널을 사용해 고루틴 간 데이터를 안전하게 교환하는 방법

	// 비버퍼링 채널 선언
	var unbufferedChannel chan int = make(chan int)

	// 버퍼링된 채널 선언 (버퍼 크기: 2)
	var bufferedChannel chan string = make(chan string, 2)

	// 채널을 사용한 동기화와 데이터 전달
	go sendDataToUnbufferedChannel(unbufferedChannel)      // 비버퍼링 채널로 데이터 송신
	go receiveDataFromUnbufferedChannel(unbufferedChannel) // 비버퍼링 채널에서 데이터 수신

	// 고루틴 간 데이터 전달 시 일정 시간 지연을 줘 동작 확인
	time.Sleep(1 * time.Second)

	// 버퍼링된 채널을 사용한 데이터 송수신 예제
	sendDataToBufferedChannel(bufferedChannel)      // 버퍼링된 채널에 데이터 송신
	receiveDataFromBufferedChannel(bufferedChannel) // 버퍼링된 채널에서 데이터 수신

	// 채널 닫기
	close(bufferedChannel) // 채널을 닫아 더 이상의 데이터 전송을 막음
	// 채널이 닫힌 후에도 반복문을 통해 남아있는 데이터 수신 가능
	for msg := range bufferedChannel {
		fmt.Printf("[채널 닫기 후 수신] %s\n", msg)
	}

}

// 비버퍼링 채널에 데이터를 송신하는 함수
func sendDataToUnbufferedChannel(ch chan int) {
	// 채널에 값 10을 송신
	fmt.Println("[비버퍼링 채널] 데이터를 송신합니다: 10")
	ch <- 10 // 비버퍼링 채널에선 수신자가 있어야 송신이 완료됨
}

// 비버퍼링 채널에서 데이터를 수신하는 함수
func receiveDataFromUnbufferedChannel(ch chan int) {
	// 채널로부터 데이터를 수신
	value := <-ch // 송신자가 있을 때 수신 가능
	fmt.Printf("[비버퍼링 채널] 데이터를 수신했습니다: %d\n", value)
}

// 버퍼링된 채널에 데이터를 송신하는 함수
func sendDataToBufferedChannel(ch chan string) {
	// 버퍼링된 채널은 지정된 버퍼 크기만큼 수신자 없이 데이터를 송신 가능
	fmt.Println("[버퍼링된 채널] 데이터를 송신합니다: Hello 1")
	ch <- "Hello 1"
	fmt.Println("[버퍼링된 채널] 데이터를 송신합니다: Hello 2")
	ch <- "Hello 2"
	fmt.Println("[버퍼링된 채널] 데이터를 송신했습니다: 모든 데이터가 버퍼에 저장되었습니다")
}

// 버퍼링된 채널에서 데이터를 수신하는 함수
func receiveDataFromBufferedChannel(ch chan string) {
	// 채널에서 데이터를 수신
	fmt.Printf("[버퍼링된 채널] 데이터를 수신합니다: %s\n", <-ch)
	fmt.Printf("[버퍼링된 채널] 데이터를 수신합니다: %s\n", <-ch)
}

// 비버퍼링 채널은 송신자가 데이터를 송신할 때 반드시 수신자가 있어야 함
// 그렇지 않으면 송신 고루틴은 대기 상태에 머무르게 됨

// 버퍼링된 채널은 지정된 버퍼 크기만큼 데이터 저장 가능, 수신자가 없더라도 데이터 송신 가능
// 버퍼가 가득 찬 경우엔 송신 고루틴이 대기 상태에 놓임

// 채널 닫기를 통해 더 이상 데이터를 송신하지 않음을 명시할 수 있음
// 닫힌 후에도 남아있는 데이터를 수신 가능

// 반복문과 range를 사용해 채널에 닫힐 때까지 데이터를 계속 수신할 수 있음
