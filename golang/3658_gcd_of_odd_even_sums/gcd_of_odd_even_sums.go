// Package main ...
package main

import "fmt"

func gcdOfOddEvenSums(n int) int {
	return n
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{4, 4},
		{5, 5},
	}

	for _, test := range tests {
		got := gcdOfOddEvenSums(test.n)
		if got != test.ans {
			fmt.Printf("gcdOfOddEvenSums(%d) = %d, want %d\n", test.n, got, test.ans)
		}
	}
}
