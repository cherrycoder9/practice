package main

import (
	"fmt"
	"sync"
)

// 뮤텍스는 여러 고루틴이 동시에 공유 자원에 접근시 발생할 수 있는 경쟁 상태를 방지하기 위해 사용됨
// 공유 자원에 대한 접근을 상호 배제함
// 고루틴이 공유 자원에 접근하기 전에 뮤텍스를 잠그고, 작업이 끝난 후 해제함
// 동시에 여러 고루틴이 자원에 접근할때 발생할 수 있는 문제를 방지함

// 공유 자원 구조체
type Counter struct {
	mu    sync.Mutex
	value int
}

// 카운터 증가 메서드
func (c *Counter) Increment(id int, wg *sync.WaitGroup) {
	// Done(): 고루틴이 완료되었음을 웨이트 그룹에 알림
	// 내부적으로 Add(-1)과 동일한 효과
	defer wg.Done()

	// 뮤텍스 잠금
	// 잠금이 성공하면 현재 고루틴은 뮤텍스를 소유하게 됨
	// 이미 잠긴 경우 다른 고루틴은 뮤텍스가 해제될 때까지 대기함
	c.mu.Lock()

	// 공유 자원 수정
	c.value++
	fmt.Printf("고루틴 %d: 카운터 값 = %d\n", id, c.value)

	// 뮤텍스 해제
	c.mu.Unlock()

	// time.Sleep(10 * time.Millisecond)
}

func Zero4() {
	var wg sync.WaitGroup
	counter := Counter{}

	// 5개의 고루틴 생성
	for i := 0; i < 5; i++ {
		// Add(delta int): delta는 양수 또는 음수, 고루틴의 시작과 완료를 관리
		wg.Add(1) // 웨이트 그룹에 추가할 고루틴의 수를 지정
		go counter.Increment(i, &wg)
	}

	// 웨이트 그룹에 지정된 모든 고루틴이 완료될 때까지 현재 고루틴을 block
	wg.Wait()

	fmt.Printf("최종 카운터 값: %d\n", counter.value)
}
