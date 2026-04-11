// Package main ...
package main

import (
	"fmt"
)

func minimumDistance(nums []int) int {
	n := len(nums)
	next := make([]int, n)
	for i := range next {
		next[i] = -1
	}
	occur := make(map[int]int)
	ans := n + 1

	for i := n - 1; i >= 0; i-- {
		if val, ok := occur[nums[i]]; ok {
			next[i] = val
		}
		occur[nums[i]] = i
	}

	for i := 0; i < n; i++ {
		secondPos := next[i]
		if secondPos != -1 {
			thirdPos := next[secondPos]
			if thirdPos != -1 {
				if dist := thirdPos - i; dist < ans {
					ans = dist
				}
			}
		}
	}

	if ans == n+1 {
		return -1
	}
	return ans * 2
}

func main() {
	tests := []struct {
		nums []int
		ans  int
	}{
		{[]int{1, 2, 1, 1, 3}, 6},
		{[]int{1, 1, 2, 3, 2, 1, 2}, 8},
		{[]int{1}, -1},
	}

	for index, test := range tests {
		if minimumDistance(test.nums) != test.ans {
			fmt.Errorf("%d", index)
		}
	}
}
