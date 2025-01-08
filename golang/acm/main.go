package main

import (
	"fmt"
	"strings"
)

func main() {
	url := "https://www.naver.com"

	if strings.HasPrefix(url, "https://") {
		fmt.Println("암호화된 프로토콜입니다.")
	}
}
