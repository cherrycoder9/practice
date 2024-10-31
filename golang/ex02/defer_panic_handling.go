package main

import (
	"fmt"
	"os"
)

// defer는 특정 함수나 코드 블록의 실행을 지연해 함수가 종료되기 직전에 실행하도록 함
// Go에서는 주로 리소스 해제, 파일 닫기, 락 해제 등과 같은 작업을 위해 사용
func basicDeferUsage() {
	fmt.Println("함수 시작")
	defer fmt.Println("defer 호출: 함수 종료 전 실행")
	fmt.Println("함수 중간 실행")
}

// 여러 개의 defer 호출 순서
// defer는 여러 번 호출될 경우 스택처럼 LIFO 순서로 실행됨
func multipleDefer() {
	fmt.Println("여러 defer 호출")
	defer fmt.Println("첫 번째 defer")
	defer fmt.Println("두 번째 defer")
	defer fmt.Println("세 번째 defer")
	fmt.Println("함수 끝")
}

// 함수 종료시 리소스 해제를 위한 defer
// 파일, 네트워크 연결, DB 연결 등의 리소스를 해제하기 위해 defer 사용
func resourceDeferUsage() {
	file, err := os.Open("example.txt")
	if err != nil {
		fmt.Println("파일 열기 실패:", err)
		return
	}
	// 파일이 정상적으로 열렸으면 함수 종료시 파일을 닫도록 함
	defer file.Close()
	fmt.Println("파일 열림", file.Name())
	// 파일 작업 수행
}

// defer와 panic/recover 조합 사용
// 예외적인 상황에서도 리소스를 안전하게 해제하기 위해 defer와 panic/recover 사용
func panicAndRecover() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("복구된 패닉:", r)
		}
	}()

	fmt.Println("panic 이전 실행")
	panic("의도적인 패닉 발생")
	fmt.Println("panic 이후 실행") // 실행되지 않음
}

// defer를 사용한 코드 가독성 향상
// 리소스 해제 코드를 별도로 작성하지 않고 defer를 통해 관리함으로써 코드 가독성 개선
func readableCodeWithDefer() {
	file, err := os.Create("output.txt")
	if err != nil {
		fmt.Println("파일 생성 실패:", err)
		return
	}
	defer file.Close()

	// 파일에 데이터 쓰기
	fmt.Fprintln(file, "Hello World!")
	fmt.Println("파일에 데이터 작성 완료")
}

// 네트워크 소켓 연결 관리
func networkConnectionExample() {
	fmt.Println("네트워크 연결 시뮬레이션 시작")
	// 가상의 네트워크 연결을 위한 구조체
	type Connection struct{}

	Connect := func(c *Connection) {
		fmt.Println("연결됨")
	}

	Close := func(c *Connection) {
		fmt.Println("연결 종료")
	}

	conn := &Connection{}
	Connect(conn)
	defer Close(conn)

	// 네트워크 연결로 작업 수행
	fmt.Println("데이터 송신 중...")
}

func deferPanicHandling() {
	fmt.Println("defer 키워드 사용 예제 시작")
	basicDeferUsage()
	fmt.Println("-------------------------")
	multipleDefer()
	fmt.Println("-------------------------")
	resourceDeferUsage()
	fmt.Println("-------------------------")
	panicAndRecover()
	fmt.Println("-------------------------")
	readableCodeWithDefer()
	fmt.Println("-------------------------")
	networkConnectionExample()
	fmt.Println("-------------------------")
}
