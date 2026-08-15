// Package main ...
package main

import "fmt"

func longestSubsequence(nums []int) int {
	n := len(nums)
	totalXor := 0
	allZero := true

	for _, x := range nums {
		totalXor ^= x
		if x > 0 {
			allZero = false
		}
	}

	if totalXor > 0 {
		return n
	}

	if allZero {
		return 0
	}
	return n - 1
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 3}, 2},
		{[]int{2, 3, 4}, 3},
	}

	for index, test := range tests {
		result := longestSubsequence(test.nums)
		if result != test.ans {
			fmt.Printf("Test %d failed: expected %d, got %d\n", index, test.ans, result)
		} else {
			fmt.Printf("Test %d passed\n", index)
		}
	}
}
