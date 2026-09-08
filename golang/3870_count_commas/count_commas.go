// Package main ...
package main

import "fmt"

func countCommas(n int) int {
	return max(n-999, 0)
}

func main() {
	tests := []struct {
		n   int
		ans int
	}{
		{1002, 3},
		{998, 0},
	}

	for index, test := range tests {
		if countCommas(test.n) != test.ans {
			fmt.Printf("failed %d\n", index)
		}
	}
}
