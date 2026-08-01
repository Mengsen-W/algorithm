// Package main ...
package main

import "fmt"

func predictTheWinner(nums []int) bool {
	max := func(x, y int) int {
		if x > y {
			return x
		}
		return y
	}
	n := len(nums)
	dp := make([]int, n)

	for i := n - 1; i >= 0; i-- {
		dp[i] = nums[i]
		for j := i + 1; j < n; j++ {
			dp[j] = max(nums[i]-dp[j], nums[j]-dp[j-1])
		}
	}
	return dp[n-1] >= 0
}

func main() {
	tests := []struct{
		nums []int
		ans bool
	}{
      {[]int{1, 5, 2}, false},
      {[]int{1, 5, 233, 7}, true},
	}

	for index, test := range tests {
		if predictTheWinner(test.nums) != test.ans {
			fmt.Println(index)
		}
	}
}
