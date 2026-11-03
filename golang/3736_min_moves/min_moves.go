// Package main ...
package main

import "fmt"

func minMoves(nums []int) int {
	mx := nums[0]
	for _, x := range nums {
		mx = max(mx, x)
	}
	ans := 0
	for _, x := range nums {
		ans += mx - x
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{2, 1, 3}, 3},
		{[]int{4, 4, 5}, 2},
	}

	for index, test := range tests {
		if minMoves(test.nums) != test.ans {
			fmt.Printf("Test %d failed", index)
		}
	}
}
