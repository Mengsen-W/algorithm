// Package main ...
package main

import "fmt"

func longestCommonPrefix(arr1 []int, arr2 []int) int {
	seen := make(map[int]bool)

	for _, num := range arr1 {
		current := num
		for current > 0 {
			seen[current] = true
			current /= 10
		}
	}

	best := 0
	for _, num := range arr2 {
		current := num
		for current > 0 {
			if seen[current] {
				best = max(best, current)
			}
			current /= 10
		}
	}

	if best == 0 {
		return 0
	}
	return len(fmt.Sprintf("%d", best))
}

func main() {
	tests := []struct {
		arr1 []int
		arr2 []int
		ans  int
	}{
		{[]int{1, 10, 100}, []int{1000}, 3},
		{[]int{1, 2, 3}, []int{4, 4, 4}, 0},
	}
	for _, test := range tests {
		if got := longestCommonPrefix(test.arr1, test.arr2); got != test.ans {
			fmt.Printf("FAIL: expected %d, got %d\n", test.ans, got)
		}
	}
}
