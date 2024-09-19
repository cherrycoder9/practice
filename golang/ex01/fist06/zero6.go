package main

import (
	"context"
	"fmt"
	"time"
)

// 파일 다운로드 시뮬레이션과 취소 기능 구현

// 파일 다운로드를 시뮬레이션하는 함수
func simulateFileDownload(ctx context.Context, fileID int) {
	fmt.Printf("파일 %d 다운로드 시작됨.\n", fileID) // 다운로드 시작 메시지 출력

	// 다운로드 작업을 5초로 시뮬레이션
	select {
	case <-time.After(5 * time.Second): // 5초 후 다운로드 완료
		fmt.Printf("파일 %d 다운로드 완료됨.\n", fileID) // 다운로드 완료 메시지 출력
	case <-ctx.Done(): // 취소 신호 수신 시
		fmt.Printf("파일 %d 다운로드 취소됨: %s\n", fileID, ctx.Err()) // 취소 메시지 출력
	}
}

func Zero6() {
	// 취소 가능한 컨텍스트 생성
	ctx, cancelFunc := context.WithCancel(context.Background())
	defer cancelFunc() // 메인 함수 종료 시 취소 함수 호출 예약

	// 파일 ID 설정
	fileID := 101 // 다운로드할 파일의 ID 설정

	// 고루틴에서 파일 다운로드 시작
	go simulateFileDownload(ctx, fileID)

	// 3초 대기
	time.Sleep(3 * time.Second)

	// 다운로드 취소 신호 전송
	fmt.Println("다운로드 취소 신호 전송됨.") // 취소 신호 전송 메시지 출력
	cancelFunc()                   // 취소 함수 호출해 컨텍스트 취소

	// 고루틴이 종료될 시간을 확보하기 위해 추가 대기
	time.Sleep(1 * time.Second)
}
