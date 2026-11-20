// Package main ...
package main

import (
	"fmt"
	"sort"
)

func maxSelectedElements(nums []int) int {
	sort.Ints(nums)
	dp := map[int]int{}
	ans := 0
	for _, x := range nums {
		dp[x+1] = max(dp[x+1], dp[x]+1)
		dp[x] = max(dp[x], dp[x-1]+1)
		ans = max(ans, dp[x], dp[x+1])
	}
	return ans
}

func main() {
	tests := []struct {
		nums []int
		want int
	}{
		{[]int{2, 1, 5, 1, 1}, 3},
		{[]int{1, 4, 7, 10}, 1},
	}

	for _, tt := range tests {
		if got := maxSelectedElements(tt.nums); got != tt.want {
			panic(fmt.Errorf("maxSelectedElements(%v) = %v, want %v", tt.nums, got, tt.want))
		}
	}
}
