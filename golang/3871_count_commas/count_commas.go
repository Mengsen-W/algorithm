// Package main ...
package main

import "fmt"

func countCommas(n int64) int64 {
	var p int64 = 1000
	var res int64 = 0
	for p <= n {
		res += n - p + 1
		p *= 1000
	}
	return res
}

func main() {
	tests := []struct {
		n        int64
		expected int64
	}{
		{1002, 3},
		{998, 0},
	}

	for index, test := range tests {
		if countCommas(test.n) != test.expected {
			panic(fmt.Sprintf("test %d failed", index))
		}
	}
}
