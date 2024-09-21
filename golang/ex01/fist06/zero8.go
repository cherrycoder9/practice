package main

import (
	"context"
	"fmt"
	"time"
)

// 데이터 처리 파이프라인에서의 취소 전파
// 상위 단계에서 취소 신호가 발생하면 하위 단계로 취소가 전파되어 모든 작업이 중단

// 데이터 생성 단계에서 생성된 데이터를 전달할 채널의 타입 정의
type Data struct {
	ID   int    // 데이터의 고유 식별자
	Info string // 데이터으 ㅣ내용 정보
}

// generateData 함수는 데이터 생성 단계를 시뮬레이션, 생성된 데이터를 채널을 통해 전달
// ctx: 컨텍스트 객체로 취소 신호를 수신하기 위해 사용
// dataCh: 생성된 데이터를 전송할 채널
func generateData(ctx context.Context, dataCh chan<- Data) {
	defer close(dataCh) // 데이터 생성이 완료되면 채널을 닫음

	for i := 1; i <= 5; i++ {
		select {
		case <-ctx.Done():
			// 취소 신호를 수신한 경우
			fmt.Printf("데이터 생성 취소됨: %s\n", ctx.Err())
			return
		default:
			// 데이터 생성 시뮬레이션
			data := Data{
				ID:   i,
				Info: fmt.Sprintf("데이터 정보 %d", i),
			}
			dataCh <- data // 생성된 데이터를 채널에 전송
			fmt.Printf("데이터 생성됨: ID=%d, Info=%s\n", data.ID, data.Info)
			time.Sleep(1 * time.Second) // 데이터 생성 간격을 위해 대기
		}
	}
}

// processData 함수는 데이터 처리 단계를 시뮬레이션, 처리된 데이터를 다음 단계로 전달
// processedCh: 처리된 데이터를 전송할 채널
func processData(ctx context.Context, dataCh <-chan Data, processedCh chan<- Data) {
	defer close(processedCh) // 데이터 처리가 완료되면 채널을 닫음

	for data := range dataCh {
		select {
		case <-ctx.Done():
			// 취소 신호를 수신한 경우
			fmt.Printf("데이터 처리 취소됨: %s\n", ctx.Err())
			return
		default:
			// 데이터 처리 시뮬레이션
			processedData := Data{
				ID:   data.ID,
				Info: data.Info + " - 처리됨",
			}
			processedCh <- processedData // 처리된 데이터를 채널에 전송
			fmt.Printf("데이터 처리됨: ID=%d, Info=%s\n", processedData.ID, processedData.Info)
			time.Sleep(1 * time.Second) // 데이터 처리 간격을 위해 대기
		}
	}
}

// saveData 함수는 데이터 저장 단계를 시뮬레이션, 처리된 데이터를 저장
func saveData(ctx context.Context, processedCh <-chan Data) {
	for data := range processedCh {
		select {
		case <-ctx.Done():
			// 취소 신호를 수신한 경우
			fmt.Printf("데이터 저장 취소됨: %s\n", ctx.Err())
			return
		default:
			// 데이터 저장 시뮬레이션
			fmt.Printf("데이터 저장됨: ID=%d, Info=%s\n", data.ID, data.Info)
			time.Sleep(1 * time.Second)
		}
	}

}

func Zero8() {
	// 취소 가능한 컨텍스트 생성
	ctx, cancel := context.WithCancel(context.Background())
	// defer cancel() // 메인 함수 종료시 컨텍스트 취소 (이미 취소 예정이므로 주석 처리)

	// 데이터 생성 단계와 데이터 처리 단계, 데이터 저장 단계를 연결할 채널 생성
	dataCh := make(chan Data)      // 데이터 생성 -> 데이터 처리
	processedCh := make(chan Data) // 데이터 처리 -> 데이터 저장

	// 데이터 생성 고루틴 실행
	go generateData(ctx, dataCh)

	// 데이터 처리 고루틴 실행
	go processData(ctx, dataCh, processedCh)

	// 데이터 저장 고루틴 실행
	go saveData(ctx, processedCh)

	// 4초 후에 취소 신호 전송
	go func() {
		time.Sleep(4 * time.Second)
		fmt.Println("전체 파이프라인 취소 신호 전송됨")
		cancel() // 컨텍스트 취소 호출
	}()

	// 전체 파이프라인 작업이 완료되거나 취소될 때까지 대기
	// 여기서는 충분한 시간 대기를 통해 프로그램이 종료되지 않도록 함
	time.Sleep(6 * time.Second)
	fmt.Println("메인 함수 종료됨")
}
