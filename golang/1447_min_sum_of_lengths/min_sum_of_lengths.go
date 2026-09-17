// Package main ...
package main

import "fmt"

func minSumOfLengths(arr []int, target int) int {
	n, ans, sum := len(arr), len(arr)+1, 0
	dp := make([]int, n+1)
	for i := range dp {
		dp[i] = n
	}
	left := 0
	for right, x := range arr {
		sum += x
		for sum > target {
			sum -= arr[left]
			left++
		}
		dp[right+1] = dp[right]
		if sum == target {
			length := right - left + 1
			if length+dp[left] < ans {
				ans = length + dp[left]
			}
			if length < dp[right+1] {
				dp[right+1] = length
			}
		}
	}
	if ans == n+1 {
		return -1
	}
	return ans
}

func main() {
	tests := []struct {
		arr    []int
		target int
		ans    int
	}{
		{[]int{3, 2, 2, 4, 3}, 3, 2},
		{[]int{7, 3, 4, 7}, 7, 2},
		{[]int{3, 2, 2, 4, 3}, 6, -1},
		{[]int{5, 5, 4, 4, 5}, 3, -1},
		{[]int{3, 1, 1, 1, 5, 1, 2, 1}, 3, 3},
	}

	for index, test := range tests {
		if minSumOfLengths(test.arr, test.target) != test.ans  {
			fmt.Println(index)
		}
	}
}
