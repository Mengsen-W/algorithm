// Package main ...
package main

import "fmt"

func climbStairs(n int, costs []int) int {
	a, b, c := 0, 0, 0
	for i := 0; i < n; i++ {
		nxt := min(min(a+9, b+4), c+1) + costs[i]
		a, b, c = b, c, nxt
	}
	return c
}

func main() {
	tests := []struct {
		n     int
		costs []int
		ans   int
	}{
		{4, []int{1, 2, 3, 4}, 13},
		{4, []int{5, 1, 6, 2}, 11},
		{3, []int{9, 8, 3}, 12},
	}

	for index, test := range tests {
		if ans := climbStairs(test.n, test.costs); ans != test.ans {
			panic(fmt.Sprintf("test %d failed", index))
		}
	}
}
