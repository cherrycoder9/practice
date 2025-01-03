package main

import "fmt"

func main() {
	var a, b int
	_, _ = fmt.Scan(&a, &b)

	if a > 0 {
		if b > 0 {
			fmt.Println("1")
		} else {
			fmt.Println("4")
		}
	} else {
		if b > 0 {
			fmt.Println("2")
		} else {
			fmt.Println("3")
		}
	}
}
