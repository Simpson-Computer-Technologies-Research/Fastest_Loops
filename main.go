package main

import (
	"fmt"
	"time"
)

// Main function
func main() {
	var data []int = []int{}
	for i := 0; i < 10_000_000; i++ {
		data = append(data, i)
	}
	WhileLoop(data)
	CountedLoop(data)
	RangeLoop(data)
}

// WhileLoop
func WhileLoop(data []int) {
	var (
		startTime time.Time = time.Now()
		i         int       = 0
	)
	for i < len(data) {
		i++
	}
	fmt.Printf("\nWhile Loop: %v\n", time.Since(startTime))
}

// Counted Loop
func CountedLoop(data []int) {
	var startTime time.Time = time.Now()
	for i := 0; i < len(data); i++ {
		i += 0
	}
	fmt.Printf("\nCounted Loop: %v\n", time.Since(startTime))
}

// Range Loop
func RangeLoop(data []int) {
	var startTime time.Time = time.Now()
	for _, v := range data {
		v += 0
	}
	fmt.Printf("\nRange Loop: %v\n", time.Since(startTime))
}
