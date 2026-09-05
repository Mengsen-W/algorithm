// Package main ...
package main

import "fmt"

func firstStableIndex(nums []int, k int) int {
	n := len(nums)
	if n == 0 {
		return -1
	}

	minValue := make([]int, n)
	minValue[n-1] = nums[n-1]
	for i := n - 2; i >= 0; i-- {
		if minValue[i+1] < nums[i] {
			minValue[i] = minValue[i+1]
		} else {
			minValue[i] = nums[i]
		}
	}

	maxValue := 0
	for i := 0; i < n; i++ {
		if nums[i] > maxValue {
			maxValue = nums[i]
		}
		if maxValue-minValue[i] <= k {
			return i
		}
	}
	return -1
}

func main() {
	tests := []struct {
		nums []int
		k    int
		ans  int
	}{
		{[]int{5, 0, 1, 4}, 3, 3},
		{[]int{3, 2, 1}, 1, -1},
		{[]int{0}, 0, 0},
	}

	for _, test := range tests {
		if got := firstStableIndex(test.nums, test.k); got != test.ans {
			panic(fmt.Sprintf("test %v failed: got %v, want %v", test, got, test.ans))
		}
	}
}
