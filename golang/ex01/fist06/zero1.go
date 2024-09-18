package main

import (
	"context"
	"fmt"
	"time"
)

// 컨텍스트 패키지 (context)를 통한 취소
// context 패키지는 고루틴 간의 취소 신호 전달과 요청 범위의 값을 전달하는데 사용됨
// 취소는 주로 긴 작업이나 외부 요청의 취소 시나리오에서 활용됨
// context는 트리 구조로 연결되어 있어 상위 컨텍스트의 취소가 하위 컨텍스트에 전파됨

// * Context 생성
// context.Background() 또는 context.TODO()로 기본 컨텍스트를 생성할 수 있음
// context.WithCancel, context.WithTimeout, context.WithDeadline 등을
// 사용해 파생된 컨텍스트를 생성할 수 있음
// * 취소 함수
// context.WithCancel을 사용하면 취소 함수를 반환받아 이를 호출해 컨텍스트를 취소할 수 있음
// * 취소 신호 수신
// 고루틴 내에서 ctx.Done() 채널을 모니터링해 취소 신호를 수신하고 적절히 종료 가능
// * 타임아웃과 데드라인
// context.WithTimeout과 context.WithDeadline을 사용해 일정 시간 후 자동으로 취소되도록 설정할 수 있음

// 작업을 수행하는 함수
func performTaskCancel(ctx context.Context, taskID int) {
	fmt.Printf("작업 %d 시작됨.\n", taskID)
	select {
	case <-time.After(5 * time.Second):
		fmt.Printf("작업 %d 완료됨.\n", taskID)
	case <-ctx.Done():
		fmt.Printf("작업 %d 취소됨: %s\n", taskID, ctx.Err())
	}
}

func Zero1() {
	// 취소 가능한 컨텍스트 생성
	ctx, cancel := context.WithCancel(context.Background())

	// 고루틴에서 작업 수행
	go performTaskCancel(ctx, 1)

	// 2초 후에 취소 호출
	time.Sleep(2 * time.Second)
	fmt.Println("취소 신호 전송됨")
	cancel()

	// 고루틴이 종료될 시간을 확보
	time.Sleep(1 * time.Second)
}
