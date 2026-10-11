// Package main ...
package main

import "fmt"

func sumOfSquares(nums []int) int {
	n := len(nums)
	ans := 0
	for i := 1; i*i <= n; i++ {
		if n%i == 0 {
			ans += nums[i-1] * nums[i-1]
			if i*i != n {
				ans += nums[n/i-1] * nums[n/i-1]
			}
		}
	}
	return ans
}

func main() {
	tests := []struct {
		nums     []int
		expected int
	}{
      {[]int{1, 2, 3, 4}, 21},
      {[]int{2, 7, 1, 19, 18, 3}, 63},
	}

	for _, test := range tests {
		result := sumOfSquares(test.nums)
		if result != test.expected {
			panic(fmt.Sprintf("Expected %d, got %d", test.expected, result))
		}
	}
}
