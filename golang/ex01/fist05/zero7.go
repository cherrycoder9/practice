package main

import (
	"fmt"
	"sync"
	"time"
)

// 뮤텍스를 이용한 은행 계좌 관리
// 동시성 안전한 은행 계좌 관리 시스템 구현
// BankAccount 구조체 정의, balance 필드와 mu 뮤텍스 포함
// Deposit 메서드를 구현해 특정 금액 입금하도록 함
// Withdraw 메서드를 구현, 특정 금액 출금, 잔액 부족한 경우 출금 거부
// 메인 고루틴에서 5개의 고루틴을 생성, 각 고루틴이 임의의 입금과 출금을 수행하도록 함
// 모든 고루틴이 완료된 후, 최종 잔액 출력

// BankAccount 구조체 정의. 잔액과 뮤텍스 포함
type BankAccount struct {
	balance int        // 계좌 잔액
	mu      sync.Mutex // 동시성 제어를 위한 뮤텍스
}

// 특정 금액을 입금하는 메서드
func (a *BankAccount) Deposit(amount int, id int, wg *sync.WaitGroup) {
	defer wg.Done() // 고루틴 완료 시 WaitGroup 카운트 감소

	a.mu.Lock()                                // 뮤텍스 잠금
	a.balance += amount                        // 계좌 잔액 증가
	fmt.Printf("고루틴 %d: %d원 입금\n", id, amount) // 입금 메시지 출력
	a.mu.Unlock()                              // 뮤텍스 해제

	time.Sleep(200 * time.Millisecond) // 작업 지연
}

// 특정 금액 출금 메서드
func (a *BankAccount) Withdraw(amount int, id int, wg *sync.WaitGroup) {
	defer wg.Done() // 고루틴 완료시 WaitGroup 카운트 감소

	a.mu.Lock()              // 뮤텍스 잠금
	if a.balance >= amount { // 잔액 확인
		a.balance -= amount // 계좌 잔액 감소
		fmt.Printf("고루틴 %d: %d원 출금\n", id, amount)
	} else {
		fmt.Printf("고루틴 %d: 출금 실패 - 잔액 부족\n", id)
	}
	a.mu.Unlock() // 뮤텍스 해제

	time.Sleep(200 * time.Millisecond) // 작업 지연
}

func Zero7() {
	var wg sync.WaitGroup // WaitGroup 초기화

	account := BankAccount{balance: 0} // BankAccount 인스턴스 생성

	// 고루틴 1: 100원 입금
	wg.Add(1)                       // WaitGroup 카운트 증가
	go account.Deposit(100, 1, &wg) // 고루틴 실행

	// 고루틴 2: 50원 출금
	wg.Add(1)                       // WaitGroup 카운트 증가
	go account.Withdraw(50, 2, &wg) // 고루틴 실행

	// 고루틴 3: 200원 입금
	wg.Add(1)                       // WaitGroup 카운트 증가
	go account.Deposit(200, 3, &wg) // 고루틴 실행

	// 고루틴 4: 200원 출금
	wg.Add(1)                        // WaitGroup 카운트 증가
	go account.Withdraw(200, 4, &wg) // 고루틴 실행

	// 고루틴 5: 100원 출금
	wg.Add(1)                        // WaitGroup 카운트 증가
	go account.Withdraw(100, 5, &wg) // 고루틴 실행

	wg.Wait() // 모든 고루틴 완료 대기

	fmt.Printf("최종 잔액: %d원\n", account.balance) // 최종 잔액 출력
}
