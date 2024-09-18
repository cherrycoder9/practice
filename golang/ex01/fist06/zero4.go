package main

import (
	"context"
	"fmt"
	"time"
)

// 자식 컨텍스트에서의 취소 전파

func performTaskChildCancel(ctx context.Context, taskID int) {
	fmt.Printf("작업 %d 시작됨.\n", taskID)
	select {
	case <-time.After(6 * time.Second):
		fmt.Printf("작업 %d 완료됨.\n", taskID)
	case <-ctx.Done():
		fmt.Printf("작업 %d 취소됨: %s\n", taskID, ctx.Err())
	}
}

func Zero4() {
	// 취소 가능한 부모 컨텍스트 생성
	parentCtx, parentCacel := context.WithCancel(context.Background())

	// 부모 컨텍스트에서 파생된 자식 컨텍스트 생성
	childCtx, childCancel := context.WithCancel(parentCtx)

	// 고루틴에서 자식 컨텍스트를 사용하는 작업 수행
	go performTaskChildCancel(childCtx, 4)

	// 2초 후 부모 컨텍스트 취소
	time.Sleep(2 * time.Second)
	fmt.Println("부모 컨텍스트 취소 호출됨.")
	parentCacel()

	// 고루틴이 종료될 시간을 확보
	time.Sleep(1 * time.Second)

	// 자식 컨텍스트 별도로 취소 (이미 취소됨)
	childCancel()
}
